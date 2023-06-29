// @generated automatically by Diesel CLI.

diesel::table! {
  fleet (ship_symbol) {
    ship_symbol -> Text,
    reg_name -> Text,
    reg_faction_symbol -> Text,
    reg_role -> Text,
    nav_system_symbol -> Text,
    nav_status -> Text,
    nav_flight_mode -> Text,
    nav_waypoint_symbol -> Text,
    nav_dest_symbol -> Text,
    nav_dest_waypoint_type -> Text,
    nav_dest_system_symbol -> Text,
    nav_dest_x -> Integer,
    nav_dest_y -> Integer,
    nav_dep_symbol -> Text,
    nav_dep_waypoint_type -> Text,
    nav_dep_system_symbol -> Text,
    nav_dep_x -> Integer,
    nav_dep_y -> Integer,
    nav_departure_time -> Text,
    nav_arrival_time -> Text,
    crew_current -> Integer,
    crew_required -> Integer,
    crew_capacity -> Integer,
    crew_rotation -> Text,
    crew_moral -> Float,
    crew_wages -> Integer,
    frame_symbol -> Text,
    frame_name -> Text,
    frame_desc -> Text,
    frame_condition -> Float,
    frame_modules -> Integer,
    frame_mounts -> Integer,
    frame_fuel_capacity -> Integer,
    frame_req_power -> Nullable<Integer>,
    frame_req_crew -> Nullable<Integer>,
    frame_req_slots -> Nullable<Integer>,
    reactor_symbol -> Text,
    reactor_name -> Text,
    reactor_desc -> Text,
    reactor_condition -> Float,
    reactor_power_output -> Integer,
    reactor_req_power -> Nullable<Integer>,
    reactor_req_crew -> Nullable<Integer>,
    reactor_req_slots -> Nullable<Integer>,
    engine_symbol -> Text,
    engine_name -> Text,
    engine_desc -> Text,
    engine_condition -> Float,
    engine_speed -> Integer,
    engine_req_power -> Nullable<Integer>,
    engine_req_crew -> Nullable<Integer>,
    engine_req_slots -> Nullable<Integer>,
    cargo_capacity -> Integer,
    cargo_units -> Integer,
    fuel_current -> Integer,
    fuel_capacity -> Integer,
    fuel_consumed_amount -> Integer,
    fuel_consumed_timestamp -> Text,
  }
}

diesel::table! {
  fleet_cargo (id) {
    id -> Integer,
    ship_symbol -> Text,
    symbol -> Text,
    name -> Text,
    description -> Text,
    units -> Integer,
  }
}

diesel::table! {
  fleet_modules (id) {
    id -> Integer,
    ship_symbol -> Text,
    symbol -> Text,
    capacity -> Nullable<Integer>,
    range -> Nullable<Integer>,
    name -> Text,
    description -> Text,
    req_power -> Nullable<Integer>,
    req_crew -> Nullable<Integer>,
    req_slots -> Nullable<Integer>,
  }
}

diesel::table! {
  fleet_mounts (id) {
    id -> Integer,
    ship_symbol -> Text,
    symbol -> Text,
    deposits -> Nullable<Text>,
    strength -> Nullable<Integer>,
    name -> Text,
    description -> Text,
    req_power -> Nullable<Integer>,
    req_crew -> Nullable<Integer>,
    req_slots -> Nullable<Integer>,
  }
}

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
  waypoints (waypoint_symbol) {
    waypoint_symbol -> Text,
    system_symbol -> Text,
    waypoint_type -> Text,
    x -> Integer,
    y -> Integer,
    orbitals -> Text,
    faction -> Nullable<Text>,
    traits -> Text,
    chart_waypoint -> Nullable<Text>,
    chart_submitted_by -> Nullable<Text>,
    chart_submitted_on -> Nullable<Text>,
  }
}

diesel::table! {
  waypoint_traits (trait_symbol) {
    trait_symbol -> Text,
    name -> Text,
    description -> Text
  }
}

diesel::table! {
  systems (system_symbol) {
    rowid -> Integer,
    system_symbol -> Text,
    sector_symbol -> Text,
    system_type -> Text,
    x -> Integer,
    y -> Integer,
    factions -> Text,
  }
}

diesel::table! {
  markets (waypoint_symbol, market_type, symbol, name, description) {
    waypoint_symbol -> Text,
    market_type -> Text,
    symbol -> Text,
    name -> Text,
    description -> Text,
  }
}

diesel::table! {
  market_trade_goods (waypoint_symbol, symbol) {
    waypoint_symbol -> Text,
    symbol -> Text,
    trade_volume -> Integer,
    supply_type -> Text,
    purchase_price -> Integer,
    sell_price -> Integer,
  }
}

diesel::table! {
  market_transactions (waypoint_symbol, ship_symbol, trade_symbol, transaction_type, units, price_per_unit, total_price, timestamp) {
    waypoint_symbol -> Text,
    ship_symbol -> Text,
    trade_symbol -> Text,
    transaction_type -> Text,
    units -> Integer,
    price_per_unit -> Integer,
    total_price -> Integer,
    timestamp -> Text,
  }
}

diesel::table! {
  jump_gates (symbol, connected_symbol) {
    symbol -> Text,
    jump_range -> Integer,
    faction_symbol -> Nullable<Text>,
    connected_symbol -> Text,
    connected_sector_symbol -> Text,
    connected_system_type -> Text,
    connected_faction_symbol -> Nullable<Text>,
    connected_x -> Integer,
    connected_y -> Integer,
    connected_distance -> Integer,
  }
}

diesel::allow_tables_to_appear_in_same_query!(
  fleet,
  fleet_modules,
  fleet_mounts,
  fleet_cargo,
  systems,
  system_waypoints,
  waypoints,
  waypoint_traits,
  market_transactions,
  market_trade_goods,
  markets,
  jump_gates,
);
