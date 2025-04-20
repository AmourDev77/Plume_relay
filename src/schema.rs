// @generated automatically by Diesel CLI.

diesel::table! {
    Messages (id) {
        #[max_length = 200]
        author_key -> Nullable<Varchar>,
        #[max_length = 200]
        recipent_key -> Nullable<Varchar>,
        id -> Integer,
        createdAt -> Timestamp,
        #[max_length = 10000]
        message_content -> Varchar,
    }
}

diesel::table! {
    User (rsa_key) {
        #[max_length = 200]
        rsa_key -> Varchar,
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
