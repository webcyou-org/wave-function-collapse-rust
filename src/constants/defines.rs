use serde::{Deserialize, Serialize};
use tokio;

pub const DIM: usize = 15;
pub const JSON_FILE_NAME: &str = "tile_floor_plan.json";
pub const GAME_WIDTH: u32 = 600;
pub const GAME_HEIGHT: u32 = 600;
pub const WINDOW_TITLE: &str = "Wave Function Collapse";
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
