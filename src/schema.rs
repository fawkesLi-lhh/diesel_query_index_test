// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id1, id2) {
        id1 -> Integer,
        id2 -> Integer,
        title -> Text,
        body -> Text,
        published -> Integer,
    }
}
