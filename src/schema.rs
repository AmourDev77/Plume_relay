// @generated automatically by Diesel CLI.

diesel::table! {
    User (id) {
        id -> Integer,
        #[max_length = 100]
        username -> Varchar,
    }
}
