mod constants;
mod core;
mod utility;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use tokio;

use crate::constants::defines::*;
use crate::core::tile::Tile;
use crate::utility::error::*;

async fn load_json_data(file_name: &str) -> Result<TileListData, MyError> {
    let mut file = File::open(format!("assets/data/{}", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: TileListData = serde_json::from_str(&contents)?;
    Ok(data)
}

async fn create_tiles_from_json<'a>(
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

fn sdl_init(sdl_context: &Sdl) -> Result<Canvas<Window>, MyError> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Wave Function Collapse", GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    Ok(canvas)
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_init(&sdl_context)?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.present();
    }

    Ok(())
}
