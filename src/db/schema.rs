// @generated automatically by Diesel CLI.

diesel::table! {
    zigs (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_name -> Varchar,
        button_counter -> Integer,
        ash_counter -> Integer,
    }
}
