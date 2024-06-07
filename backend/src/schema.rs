// @generated automatically by Diesel CLI.

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 20]
        title -> Varchar,
        #[max_length = 600]
        description -> Varchar,
        #[max_length = 100]
        front_image -> Varchar,
        #[max_length = 100]
        image -> Varchar,
        duration -> Int4,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
