// @generated automatically by Diesel CLI.

diesel::table! {
    game_challenges (id) {
        id -> Uuid,
        challenger_uid -> Text,
        game_type -> Text,
        ranked -> Bool,
        public -> Bool,
        tournament_queen_rule -> Bool,
        color_choice -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (uid) {
        uid -> Text,
        username -> Varchar,
        is_guest -> Bool,
    }
}

diesel::joinable!(game_challenges -> users (challenger_uid));

diesel::allow_tables_to_appear_in_same_query!(game_challenges, users,);
