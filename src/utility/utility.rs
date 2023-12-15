use crate::constants::defines::{TileListData, GAME_HEIGHT, GAME_WIDTH, WINDOW_TITLE};
use crate::core::tile::Tile;
use crate::utility::error::MyError;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use std::fs::File;
use std::io::Read;

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn compare_edge(a: &str, b: &str) -> bool {
    a == reverse_string(b)
}

pub async fn load_json_data(file_name: &str) -> Result<TileListData, MyError> {
    let mut file = File::open(format!("assets/data/{}", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: TileListData = serde_json::from_str(&contents)?;
    Ok(data)
}

pub async fn create_tiles_from_json<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    file_name: &str,
) -> Result<Vec<Tile<'a>>, MyError> {
    let tile_list_data = load_json_data(file_name).await?;
    let mut tiles = Vec::new();

    for tile_data in tile_list_data.tile_list {
        tiles.push(Tile::load(
            texture_creator,
            tile_data.src,
            tile_data.edges,
            tile_data.is_rotate,
        )?);
    }

    Ok(tiles)
}

pub fn sdl_init(sdl_context: &Sdl) -> Result<Canvas<Window>, MyError> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(WINDOW_TITLE, GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    Ok(canvas)
}
