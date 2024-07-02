// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 255]
        author_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(blogs -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    blogs,
    users,
);
