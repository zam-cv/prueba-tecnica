use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}
