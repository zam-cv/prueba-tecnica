use crate::{config, models, schema};
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use std::env;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        );

        let pool = r2d2::Pool::builder()
            // Set the maximum number of connections to the database
            .max_size(config::MAX_POOL_SIZE)
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub fn get_connection(
        &self,
    ) -> anyhow::Result<PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn query_wrapper<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
        T: Send + 'static,
    {
        let mut conn = self.get_connection()?;

        // Execute the query
        let result = web::block(move || f(&mut conn))
            .await
            .map_err(|e| {
                log::error!("Database error: {:?}", e);
                anyhow::anyhow!(e)
            })?
            .map_err(|e| {
                log::error!("Database error: {:?}", e);
                anyhow::anyhow!(e)
            })?;

        Ok(result)
    }

    pub async fn create_user(&self, new_user: models::User) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .returning(schema::users::id)
                .get_result(conn)
        })
        .await
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<models::User>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .filter(schema::users::email.eq(email))
                .first(conn)
                .optional()
        })
        .await
    }

    pub async fn get_user_by_id(&self, id: i32) -> anyhow::Result<Option<models::User>> {
        self.query_wrapper(move |conn| schema::users::table.find(id).first(conn).optional())
            .await
    }

    pub async fn create_room(&self, new_room: models::Room) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::rooms::table)
                .values(&new_room)
                .returning(schema::rooms::id)
                .get_result(conn)
        })
        .await
    }

    pub async fn get_rooms_count(&self) -> anyhow::Result<i64> {
        self.query_wrapper(move |conn| schema::rooms::table.count().get_result(conn))
            .await
    }

    pub async fn get_rooms(&self) -> anyhow::Result<Vec<(i32, String, String, String, i32)>> {
        self.query_wrapper(move |conn| {
            schema::rooms::table
                .select((
                    schema::rooms::id,
                    schema::rooms::title,
                    schema::rooms::description,
                    schema::rooms::front_image,
                    schema::rooms::duration,
                ))
                .load(conn)
        })
        .await
    }

    pub async fn get_room_info(&self, id: i32) -> anyhow::Result<Option<(String, String, String, i32)>> {
        self.query_wrapper(move |conn| {
            schema::rooms::table
                .select((
                    schema::rooms::title,
                    schema::rooms::image,
                    schema::rooms::example,
                    schema::rooms::duration,
                ))
                .filter(schema::rooms::id.eq(id))
                .first(conn)
                .optional()
        })
        .await
    }

    pub async fn check_room_answer(&self, room_id: i32, answer: String) -> anyhow::Result<bool> {
        self.query_wrapper(move |conn| {
            schema::rooms::table
                .select(schema::rooms::answer)
                .filter(schema::rooms::id.eq(room_id))
                .first(conn)
                .map(|a: String| a == answer)
        })
        .await
    }

    pub async fn insert_solving_time(
        &self,
        solving_time: models::SolvingTime,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            if let Ok(time) = schema::solving_times::table
                .select(schema::solving_times::time)
                .filter(schema::solving_times::room_id.eq(solving_time.room_id))
                .filter(schema::solving_times::user_id.eq(solving_time.user_id))
                .first::<i32>(conn)
            {
                if time < solving_time.time {
                    return Ok(());
                }
            }

            let _ = diesel::insert_into(schema::solving_times::table)
                .values(&solving_time)
                .on_conflict((
                    schema::solving_times::room_id,
                    schema::solving_times::user_id,
                ))
                .do_update()
                .set(schema::solving_times::time.eq(solving_time.time))
                .execute(conn);

            Ok(())
        })
        .await?;

        Ok(())
    }

    pub async fn get_best_solving_time(
        &self,
        room_id: i32,
    ) -> anyhow::Result<Option<(i32, String)>> {
        self.query_wrapper(move |conn| {
            schema::solving_times::table
                .filter(schema::solving_times::room_id.eq(room_id))
                .inner_join(schema::users::table)
                .select((schema::solving_times::time, schema::users::username))
                .order(schema::solving_times::time)
                .first(conn)
                .optional()
        })
        .await
    }
}
