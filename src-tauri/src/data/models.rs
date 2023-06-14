use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data::schema::systems)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SystemDB {
  pub system_symbol: String,
  pub sector_symbol: String,
  pub system_type: String,
  pub x: i32,
  pub y: i32,
  pub waypoints: String,
  pub factions: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::systems)]
pub struct NewSystemDB<'a> {
  pub system_symbol: &'a str,
  pub sector_symbol: &'a str,
  pub system_type: &'a str,
  pub x: i32,
  pub y: i32,
  pub waypoints: &'a str,
  pub factions: &'a str
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data::schema::system_waypoints)]
pub struct SystemWaypointDB {
  pub waypoint_symbol: String,
  pub system_symbol: String,
  pub waypoint_type: String,
  pub x: i32,
  pub y: i32
}
#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::system_waypoints)]
pub struct NewSystemWaypointDB<'a> {
  pub waypoint_symbol: &'a str,
  pub system_symbol: &'a str,
  pub waypoint_type: &'a str,
  pub x: i32,
  pub y: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data::schema::fleet)]
pub struct ShipDB {
  pub ship_symbol: String,
  pub reg_name: String,
  pub reg_faction_symbol: String,
  pub reg_role: String,
  pub nav_system_symbol: String,
  pub nav_status: String,
  pub nav_flight_mode: String,
  pub nav_waypoint_symbol: String,
  pub nav_departure_time: String,
  pub nav_arrival_time: String,
  pub nav_dest_symbol: String,
  pub nav_dest_waypoint_type: String,
  pub nav_dest_system_symbol: String,
  pub nav_dest_x: i32,
  pub nav_dest_y: i32,
  pub nav_dep_symbol: String,
  pub nav_dep_waypoint_type: String,
  pub nav_dep_system_symbol: String,
  pub nav_dep_x: i32,
  pub nav_dep_y: i32,
  pub crew_current: i32,
  pub crew_required: i32,
  pub crew_capacity: i32,
  pub crew_rotation: String,
  pub crew_moral: i32,
  pub crew_wages: i32,
  pub frame_symbol: String,
  pub frame_name: String,
  pub frame_desc: String,
  pub frame_condition: f32,
  pub frame_modules: i32,
  pub frame_mounts: i32,
  pub frame_fuel_capacity: i32,
  pub frame_req_power: Option<i32>,
  pub frame_req_crew: Option<i32>,
  pub frame_req_slots: Option<i32>,
  pub reactor_symbol: String,
  pub reactor_name: String,
  pub reactor_desc: String,
  pub reactor_condition: i32,
  pub reactor_power_output: i32,
  pub reactor_req_power: Option<i32>,
  pub reactor_req_crew: Option<i32>,
  pub reactor_req_slots: Option<i32>,
  pub engine_symbol: String,
  pub engine_name: String,
  pub engine_desc: String,
  pub engine_condition: i32,
  pub engine_speed: i32,
  pub engine_req_power: Option<i32>,
  pub engine_req_crew: Option<i32>,
  pub engine_req_slots: Option<i32>,
  pub modules: String, /// List of UUIDS
  pub mounts: String, /// List of UUIDs
  pub cargo_id: String,
  pub cargo_capacity: i32,
  pub cargo_units: i32,
  pub fuel_current: i32,
  pub fuel_capacity: i32,
  pub fuel_consumed_amount: i32,
  pub fuel_consumed_timestamp: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::fleet)]
pub struct NewShipDB<'a> {
  pub ship_symbol: &'a str,
  pub reg_name: &'a str,
  pub reg_faction_symbol: &'a str,
  pub reg_role: &'a str,
  pub nav_system_symbol: &'a str,
  pub nav_status: &'a str,
  pub nav_flight_mode: &'a str,
  pub nav_waypoint_symbol: &'a str,
  pub nav_departure_time: &'a str,
  pub nav_arrival_time: &'a str,
  pub nav_dest_symbol: &'a str,
  pub nav_dest_waypoint_type: &'a str,
  pub nav_dest_system_symbol: &'a str,
  pub nav_dest_x: i32,
  pub nav_dest_y: i32,
  pub nav_dep_symbol: &'a str,
  pub nav_dep_waypoint_type: &'a str,
  pub nav_dep_system_symbol: &'a str,
  pub nav_dep_x: i32,
  pub nav_dep_y: i32,
  pub crew_current: i32,
  pub crew_required: i32,
  pub crew_capacity: i32,
  pub crew_rotation: &'a str,
  pub crew_moral: i32,
  pub crew_wages: i32,
  pub frame_symbol: &'a str,
  pub frame_name: &'a str,
  pub frame_desc: &'a str,
  pub frame_condition: f32,
  pub frame_modules: i32,
  pub frame_mounts: i32,
  pub frame_fuel_capacity: i32,
  pub frame_req_power: Option<i32>,
  pub frame_req_crew: Option<i32>,
  pub frame_req_slots: Option<i32>,
  pub reactor_symbol: &'a str,
  pub reactor_name: &'a str,
  pub reactor_desc: &'a str,
  pub reactor_condition: i32,
  pub reactor_power_output: i32,
  pub reactor_req_power: Option<i32>,
  pub reactor_req_crew: Option<i32>,
  pub reactor_req_slots: Option<i32>,
  pub engine_symbol: &'a str,
  pub engine_name: &'a str,
  pub engine_desc: &'a str,
  pub engine_condition: i32,
  pub engine_speed: i32,
  pub engine_req_power: Option<i32>,
  pub engine_req_crew: Option<i32>,
  pub engine_req_slots: Option<i32>,
  pub modules: &'a str, /// List of UUIDS
  pub mounts: &'a str, /// List of UUIDS
  pub cargo_id: &'a str,
  pub cargo_capacity: i32,
  pub cargo_units: i32,
  pub fuel_current: i32,
  pub fuel_capacity: i32,
  pub fuel_consumed_amount: i32,
  pub fuel_consumed_timestamp: &'a str
}