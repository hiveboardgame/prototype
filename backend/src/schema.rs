// @generated automatically by Diesel CLI.

diesel::table! {
    users (uid) {
        uid -> Text,
        username -> Text,
        is_guest -> Bool,
    }
}
