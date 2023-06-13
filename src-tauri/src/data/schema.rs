// @generated automatically by Diesel CLI.

diesel::table! {
    systems (symbol) {
        symbol -> Text,
        sector_symbol -> Text,
        system_type -> Text,
        x -> Integer,
        y -> Integer,
        waypoints -> Nullable<Text>,
        factions -> Nullable<Text>,
    }
}
