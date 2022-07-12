use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use pbr::ProgressBar;
use serde::Serialize;

#[derive(Serialize)]
pub struct Container {
    blueprint: Blueprint,
}

#[derive(Serialize)]
pub struct Blueprint {
    icons: Vec<Icon>,
    entities: Vec<Entity>,
    tiles: Vec<Tile>,
    item: &'static str,
    label: &'static str,
    version: u64,
}

#[derive(Serialize)]
pub struct Icon {
    signal: Signal,
    index: usize,
}

#[derive(Serialize)]
pub struct Signal {
    #[serde(rename = "type")]
    kind: SignalKind,
    name: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum SignalKind {
    Item,
    Fluid,
    Virtual,
}

#[derive(Serialize)]
pub struct Entity {
    entity_number: usize,
    name: &'static str,
    position: Position,
    #[serde(skip_serializing_if = "Direction::should_skip")]
    direction: Direction,
}

#[derive(Serialize, PartialEq)]
pub enum Direction {
    North = 0,
    _East = 2,
    South = 4,
    _West = 6,
}

impl Direction {
    fn should_skip(&self) -> bool {
        self == &Direction::North
    }
}

#[derive(Serialize)]
pub struct Tile {
    name: &'static str,
    position: Position,
}

#[derive(Serialize)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Container {
    pub fn build(entities: Vec<Vec<&'static str>>, tiles: Vec<Vec<&'static str>>) -> Self {
        let mut container = Container {
            blueprint: Blueprint {
                icons: vec![Icon {
                    signal: Signal {
                        kind: SignalKind::Item,
                        name: "stone-wall",
                    },
                    index: 1,
                }],
                entities: Vec::new(),
                tiles: Vec::new(),
                item: "blueprint",
                label: "Generated Blueprint",
                version: 281479275413505,
            },
        };

        let mut progress_bar = ProgressBar::new(entities.len() as u64 * entities.get(0).unwrap().len() as u64);

        progress_bar.message("Building entities ");

        let mut x = 0;
        let mut entity_number = 1;
        for (row_index, row) in entities.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                x += 1;
                if x == 99999 {
                    progress_bar.add(99999);
                    x = 0;
                }
                if item == &"" {
                    continue;
                }
                let direction = if item == &"pipe-to-ground" && row_index % 2 == 1 {
                    Direction::South
                } else {
                    Direction::North
                };
                container.blueprint.entities.push(Entity {
                    entity_number,
                    name: item,
                    position: Position {
                        x: column_index as f64 + 0.5,
                        y: row_index as f64 + 0.5,
                    },
                    direction,
                });
                entity_number += 1;
            }
        }

        progress_bar.add(x);
        progress_bar.finish();
        println!();

        let mut progress_bar = ProgressBar::new(tiles.len() as u64 * tiles.get(0).unwrap().len() as u64);

        progress_bar.message("Building tiles ");

        for (row_index, row) in tiles.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                x += 1;
                if x == 99999 {
                    progress_bar.add(99999);
                    x = 0;
                }
                progress_bar.inc();
                if item == &"" {
                    continue;
                }
                container.blueprint.tiles.push(Tile {
                    name: item,
                    position: Position {
                        x: column_index as f64,
                        y: row_index as f64,
                    },
                });
            }
        }

        progress_bar.add(x);
        progress_bar.finish();
        println!();

        container
    }

    pub fn encode(&self) -> String {
        println!("Encoding into JSON");
        let json_step = serde_json::to_vec(&self).unwrap();
        println!("Compressing");
        let mut zlib_compressor = ZlibEncoder::new(Vec::new(), Compression::new(9));
        zlib_compressor.write_all(&json_step).unwrap();
        let zlib_step = zlib_compressor.finish().unwrap();
        println!("Base64 encoding");
        let b64_step = base64::encode(zlib_step);
        let mut version_step = String::new();
        version_step.push('0');
        version_step.push_str(&b64_step);

        version_step
    }
}
