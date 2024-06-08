use crate::{
    database::Database,
    socket::session::{Response, Session},
};
use actix::prelude::*;
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Clone)]
pub enum Command {
    Connect(i32, i32, Addr<Session>),
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
    pub user_id: i32,
    pub room_id: i32,
    pub connected_at: std::time::Instant,
    pub session: Addr<Session>,
}

impl State {
    pub async fn new(user_id: i32, room_id: i32, session: Addr<Session>) -> anyhow::Result<State> {
        Ok(State {
            user_id,
            room_id,
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

    async fn connect(&mut self, user_id: &i32, room_id: &i32, addr: &Addr<Session>) {
        log::debug!("Connected: {}", user_id);

        // if a connection already exists, it is rejected
        if self.sessions.contains_key(user_id) {
            log::debug!("Connection already exists: {}", user_id);
            addr.do_send(Response::Stop);

            return;
        }

        if let Ok(Some(data)) = self.database.get_room_info(*room_id).await {
            if let Ok(state) = State::new(*user_id, *room_id, addr.clone()).await {
                if let Ok(info) = serde_json::to_string(&data) {
                    addr.do_send(Response::String(info));
                    self.sessions.insert(*user_id, state);
                    return;
                }
            }
        }

        log::debug!("Failed to get room info: {}", room_id);
        addr.do_send(Response::Stop);
    }

    async fn disconnect(&mut self, user_id: &i32) {
        if let Some(state) = self.sessions.remove(user_id) {
            log::debug!("Disconnected in {}: {}", state.room_id, user_id);
        }
    }

    async fn message(&mut self, user_id: &i32, text: &String) {
        log::debug!("Message from {}: {}", user_id, text);

        if let Some(state) = self.sessions.get_mut(user_id) {
            log::debug!("Message in {}: {}", state.room_id, text);
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
