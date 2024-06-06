// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
    }
}
