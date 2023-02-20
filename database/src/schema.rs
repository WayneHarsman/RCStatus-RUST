// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        target -> Text,
        remote -> Text,
    }
}
