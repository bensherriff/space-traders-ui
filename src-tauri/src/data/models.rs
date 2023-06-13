use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data::schema::systems)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SystemDB {
    pub symbol: String,
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
    pub symbol: &'a str,
    pub sector_symbol: &'a str,
    pub system_type: &'a str,
    pub x: i32,
    pub y: i32,
    pub waypoints: &'a str,
    pub factions: &'a str
}