// @generated automatically by Diesel CLI.

diesel::table! {
    posts (title) {
        title -> Text,
        created -> Timestamptz,
        modified -> Timestamptz,
    }
}
