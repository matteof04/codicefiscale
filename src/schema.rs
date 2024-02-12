// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Integer,
        city_name -> Text,
        city_code -> Text,
    }
}

diesel::table! {
    nations (id) {
        id -> Integer,
        nation_name -> Text,
        nation_code -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    nations,
);
