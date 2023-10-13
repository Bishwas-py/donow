// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        will_be_completed_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}
