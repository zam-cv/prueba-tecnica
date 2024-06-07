use crate::{
    database::Database,
    socket::session::{Response, Session},
};
use actix::prelude::*;
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Clone)]
pub enum Command {
    Connect(i32, Addr<Session>),
    Disconnect(i32),
    Message(i32, String),
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
    pub id: i32,
    pub connected_at: std::time::Instant,
    pub session: Addr<Session>,
}

impl State {
    pub async fn new(
        id: i32,
        session: Addr<Session>
    ) -> anyhow::Result<State> {
        Ok(State {
            id,
            connected_at: std::time::Instant::now(),
            session,
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

    async fn connect(&mut self, id: &i32, addr: &Addr<Session>) {
        log::debug!("Connected: {}", id);

        // if a connection already exists, it is rejected
        if self.sessions.contains_key(id) {
            log::debug!("Connection already exists: {}", id);
            addr.do_send(Response::Stop);

            return;
        }

        if let Ok(state) = State::new(*id, addr.clone()).await {
            self.sessions.insert(*id, state);
        } else {
            log::error!("Failed to get user: {}", id);
            addr.do_send(Response::Stop);
        }
    }

    async fn disconnect(&mut self, id: &i32) {
        log::debug!("Disconnected: {}", id);

        if let Some(mut state) = self.sessions.remove(id) {
            // Save the state in the database at the end of the session
            // let _ = state.save(&self.database).await;
        }
    }

    async fn message(&mut self, id: &i32, text: &String) {
        log::debug!("Message from {}: {}", id, text);

        if let Some(state) = self.sessions.get_mut(id) {
            // if let Err(err) = state.handle_message(text, &self.database).await {
            //     log::error!("Failed to handle message: {}", err);
            // }
        }
    }

    pub async fn run(&mut self) {
        while let Some(cmd) = self.rx.recv().await {
            match &cmd {
                Command::Connect(id, addr) => {
                    self.connect(id, addr).await;
                }
                Command::Disconnect(id) => {
                    self.disconnect(id).await;
                }
                Command::Message(id, text) => {
                    self.message(id, text).await;
                }
            }
        }
    }
}
