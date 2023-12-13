use serde::{Deserialize, Serialize};
use tokio;

pub const DIM: u32 = 15;
pub const JSON_FILE_NAME: &str = "tile_simple.json";
pub const GAME_WIDTH: u32 = 600;
pub const GAME_HEIGHT: u32 = 600;

#[derive(Serialize, Deserialize, Debug)]
pub struct TileData {
    pub src: String,
    pub edges: Vec<String>,
    pub is_rotate: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileListData {
    pub tile_list: Vec<TileData>,
}
