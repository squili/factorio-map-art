// (hack warning) i got lifetime errors when trying to add a deserialize impl, so instead im
// converting all the `&'static str`s to `String`s and putting them here so they don't hurt
// performance during serialization

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
pub struct Container {
    pub blueprint: Blueprint,
}

#[derive(Deserialize, Serialize)]
pub struct Blueprint {
    pub icons: Vec<Icon>,
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(default)]
    pub tiles: Vec<Tile>,
    pub item: String,
    #[serde(default)]
    pub label: String,
    pub version: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Icon {
    pub signal: Signal,
    pub index: usize,
}

#[derive(Deserialize, Serialize)]
pub struct Signal {
    #[serde(rename = "type")]
    pub kind: SignalKind,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SignalKind {
    Item,
    Fluid,
    Virtual,
}

#[derive(Deserialize, Serialize)]
pub struct Entity {
    pub entity_number: usize,
    pub name: String,
    pub position: Position,
}

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub name: String,
    pub position: Position,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
