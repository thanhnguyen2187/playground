// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Nullable<Text>,
        title -> Text,
        completed -> Bool,
    }
}
