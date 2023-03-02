// @generated automatically by Diesel CLI.

diesel::table! {
    users (uid) {
        uid -> Text,
        username -> Varchar,
        is_guest -> Bool,
    }
}
