use crate::schema;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::rooms)]
pub struct Room {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    pub title: String,
    #[validate(length(min = 1, max = 600))]
    pub description: String,
    #[validate(length(min = 1, max = 100))]
    pub front_image: String,
    #[validate(length(min = 1, max = 100))]
    pub image: String,
    pub duration: i32,
    #[validate(length(max = 100))]
    pub example: String,
    #[validate(length(min = 1, max = 200))]
    pub answer: String,
}
