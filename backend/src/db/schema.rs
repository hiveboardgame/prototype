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
    games (id) {
        id -> Int4,
        black_uid -> Text,
        game_status -> Text,
        game_type -> Text,
        history -> Text,
        game_control_history -> Text,
        ranked -> Bool,
        tournament_queen_rule -> Bool,
        turn -> Int4,
        white_uid -> Text,
    }
}

diesel::table! {
    games_users (game_id, user_uid) {
        game_id -> Int4,
        user_uid -> Text,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        user_uid -> Text,
        game_type -> Text,
        games_played -> Nullable<Int8>,
        turn_based -> Float8,
        puzzle -> Float8,
        updated_at -> Timestamptz,
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
diesel::joinable!(games_users -> games (game_id));
diesel::joinable!(games_users -> users (user_uid));
diesel::joinable!(ratings -> users (user_uid));

diesel::allow_tables_to_appear_in_same_query!(
    game_challenges,
    games,
    games_users,
    ratings,
    users,
);
