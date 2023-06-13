// @generated automatically by Diesel CLI.

diesel::table! {
    system_waypoints (waypoint_symbol) {
        waypoint_symbol -> Text,
        system_symbol -> Text,
        waypoint_type -> Text,
        x -> Integer,
        y -> Integer,
    }
}

diesel::table! {
    systems (system_symbol) {
        system_symbol -> Text,
        sector_symbol -> Text,
        system_type -> Text,
        x -> Integer,
        y -> Integer,
        waypoints -> Text,
        factions -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    system_waypoints,
    systems,
);
