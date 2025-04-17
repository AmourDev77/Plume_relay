// @generated automatically by Diesel CLI.

diesel::table! {
    Messages (id) {
        id -> Integer,
        #[max_length = 100]
        author -> Varchar,
        createdAt -> Timestamp,
        #[max_length = 10000]
        message_content -> Varchar,
    }
}

diesel::table! {
    User (id) {
        id -> Integer,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 200]
        password -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        createdAt -> Timestamp,
        #[max_length = 400]
        banner -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    Messages,
    User,
);
