// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}
