use crate::{
    database::Database,
    socket::{self, session::Session},
    models
};
use actix::prelude::*;
use serde::Serialize;
use std::{collections::HashMap, time::Instant};
use tokio::{
    sync::{mpsc, oneshot},
    time::{self, Duration},
};

#[derive(Clone)]
pub enum Command {
    Connect(i32, i32, Addr<Session>),
    Disconnect(i32),
    Message(i32, String),
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Response {
    Init {
        title: String,
        image: String,
        example: String,
        duration: i32,
    },
    Win,
    Lose,
}

#[derive(Clone)]
pub struct ServerHandle {
    pub tx: mpsc::UnboundedSender<Command>,
}

pub struct Server {
    sessions: HashMap<i32, State>,
    database: Database,
    rx: mpsc::UnboundedReceiver<Command>,
}

pub struct State {
    pub user_id: i32,
    pub room_id: i32,
    pub session: Addr<Session>,
    pub stop: Option<oneshot::Sender<()>>,
    pub start_time: Instant,
}

// room timer
async fn timer(duration: Duration, cancel: oneshot::Receiver<()>, addr: Addr<Session>) {
    tokio::select! {
        _ = time::sleep(duration) => {
            log::debug!("Timer expired");

            // send a message to the user that they lost
            let _ = Server::send(&addr, Response::Lose);

            // stop the session
            addr.do_send(socket::session::Response::Stop);
        }
        _ = cancel => {
            log::debug!("Timer cancelled");
        }
    };
}

impl State {
    pub async fn new(
        user_id: i32,
        room_id: i32,
        session: Addr<Session>,
        stop: oneshot::Sender<()>,
    ) -> anyhow::Result<State> {
        Ok(State {
            user_id,
            room_id,
            session,
            stop: Some(stop),
            start_time: Instant::now(),
        })
    }
}

impl Server {
    pub fn new(database: Database) -> (Self, ServerHandle) {
        let (tx, rx) = mpsc::unbounded_channel();

        (
            Server {
                sessions: HashMap::new(),
                database,
                rx,
            },
            ServerHandle { tx },
        )
    }

    // Send a message to the user
    pub fn send(session: &Addr<Session>, response: Response) -> anyhow::Result<()> {
        let response = serde_json::to_string(&response)?;
        session.do_send(socket::session::Response::String(response));
        Ok(())
    }

    async fn connect(&mut self, user_id: &i32, room_id: &i32, addr: &Addr<Session>) {
        log::debug!("Connected: {}", user_id);

        // if a connection already exists, it is rejected
        if self.sessions.contains_key(user_id) {
            log::debug!("Connection already exists: {}", user_id);
            addr.do_send(socket::session::Response::Stop);

            return;
        }

        if let Ok(Some(data)) = self.database.get_room_info(*room_id).await {
            let duration = data.3;
            let (tx, rx) = oneshot::channel();
            let time = Duration::from_secs(duration as u64 * 60);
            let timer_handle = tokio::spawn(timer(time, rx, addr.clone()));

            // start the timer
            tokio::spawn(async move {
                let _ = timer_handle.await;
            });

            if let Ok(state) = State::new(*user_id, *room_id, addr.clone(), tx).await {
                if let Ok(()) = Self::send(
                    addr,
                    Response::Init {
                        title: data.0,
                        image: data.1,
                        example: data.2,
                        duration,
                    },
                ) {
                    // save the session
                    self.sessions.insert(*user_id, state);
                    return;
                }
            }
        }

        log::debug!("Failed to get room info: {}", room_id);
        addr.do_send(socket::session::Response::Stop);
    }

    async fn disconnect(&mut self, user_id: &i32) {
        if let Some(state) = self.sessions.remove(user_id) {
            log::debug!("Disconnected in {}: {}", state.room_id, user_id);

            // cancel the timer
            if let Some(stop) = state.stop {
                if stop.send(()).is_ok() {
                    log::debug!("Timer cancelled for user: {}", user_id);
                }
            }
        }
    }

    async fn message(&mut self, user_id: &i32, text: &String) {
        if let Some(state) = self.sessions.get_mut(user_id) {
            if let Ok(true) = self
                .database
                .check_room_answer(state.room_id, text.clone())
                .await
            {
                log::debug!("Correct answer: {}", text);

                if let Some(stop) = state.stop.take() {
                    if stop.send(()).is_ok() {
                        log::debug!("Timer cancelled for user: {}", user_id);

                        // calculate the elapsed time
                        let elapsed_time = state.start_time.elapsed();
                        let seconds_elapsed = elapsed_time.as_secs();

                        log::debug!("Elapsed time: {}", seconds_elapsed);

                        // send a message to the user that they won
                        let _ = Self::send(&state.session, Response::Win);

                        // insert the solving time into the database
                        let _ = self.database
                            .insert_solving_time(models::SolvingTime {
                                room_id: state.room_id,
                                user_id: state.user_id,
                                time: seconds_elapsed as i32,
                            })
                            .await;
                    }
                }
            }
        }
    }

    pub async fn run(&mut self) {
        while let Some(cmd) = self.rx.recv().await {
            match &cmd {
                Command::Connect(user_id, room_id, addr) => {
                    self.connect(user_id, room_id, addr).await;
                }
                Command::Disconnect(user_id) => {
                    self.disconnect(user_id).await;
                }
                Command::Message(user_id, text) => {
                    self.message(user_id, text).await;
                }
            }
        }
    }
}
