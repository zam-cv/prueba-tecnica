use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(room_id, user_id))]
#[diesel(table_name = schema::solving_times)]
#[diesel(belongs_to(Room))]
#[diesel(belongs_to(User))]
pub struct SolvingTime {
    pub room_id: i32,
    pub user_id: i32,
    pub time: i32,
}
