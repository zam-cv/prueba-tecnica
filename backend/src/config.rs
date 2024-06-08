use crate::{database::Database, models};
use lazy_static::lazy_static;
use validator::Validate;
use std::env;

lazy_static! {
    pub static ref SECRET_KEY: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}

// default
const ROOMS: &str = include_str!("../default/rooms.json");

// constants
pub const TOKEN_EXPIRATION_TIME: usize = 60 * 60 * 24 * 15; // 15 days
pub const MAX_POOL_SIZE: u32 = 5; // Database connection pool size

pub async fn create_rooms(database: &Database) -> anyhow::Result<()> {
    // Check if there are rooms in the database
    if database.get_rooms_count().await? > 0 {
        return Ok(());
    }

    // Load the rooms from the JSON file
    let rooms = serde_json::from_str::<Vec<models::Room>>(ROOMS)?;

    for room in rooms {
        room.validate()?;
        database.create_room(room).await?;
    }

    Ok(())
}

pub async fn configure_database(database: &Database) {
    // Create the rooms
    create_rooms(database).await.unwrap();
}