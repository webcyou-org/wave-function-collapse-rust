mod constants;
mod core;
mod utility;

use crate::constants::defines::*;
use crate::core::cell::Cell;
use crate::core::tile::Tile;
use crate::core::wfc::{pick_cell_with_least_entropy, random_selection_of_sockets, wave_collapse};
use crate::utility::error::*;
use crate::utility::utility::{create_tiles_from_json, sdl_init};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_init(&sdl_context)?;
    let texture_creator = canvas.texture_creator();

    let tiles = init_tiles(&texture_creator).await?;
    let mut grid: Vec<Cell> = init_grid(tiles.len());

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    'running: loop {
        main_loop(&mut grid, &tiles);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    grid = init_grid(tiles.len());
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw(&mut canvas, &grid, &tiles);
        canvas.present();
    }
    Ok(())
}

async fn init_tiles(texture_creator: &TextureCreator<WindowContext>) -> Result<Vec<Tile>, MyError> {
    let mut tiles = create_tiles_from_json(&texture_creator, JSON_FILE_NAME).await?;
    create_rotate_tiles(&mut tiles);
    generating_adjacency_rules(&mut tiles);
    Ok(tiles)
}

fn init_grid(length: usize) -> Vec<Cell> {
    (0..DIM * DIM)
        .map(|_index| Cell::from_value(length))
        .collect()
}

fn draw(canvas: &mut Canvas<Window>, grid: &[Cell], tiles: &[Tile]) {
    let w = GAME_WIDTH / DIM as u32;
    let h = GAME_HEIGHT / DIM as u32;

    for j in 0..DIM {
        for i in 0..DIM {
            let index = i + j * DIM;
            let cell = &grid[index];
            if cell.collapsed {
                let tile_index = cell.sockets[0];
                let tile = &tiles[tile_index];
                tile.render(
                    canvas,
                    (i as u32 * w).try_into().unwrap(),
                    (j as u32 * h).try_into().unwrap(),
                    w,
                    h,
                );
            }
        }
    }
}

fn main_loop(grid: &mut Vec<Cell>, tiles: &[Tile]) {
    let mut low_entropy_grid = pick_cell_with_least_entropy(grid);
    if low_entropy_grid.is_empty() {
        return;
    }
    if !random_selection_of_sockets(&mut low_entropy_grid) {
        *grid = init_grid(tiles.len());
        return;
    }
    wave_collapse(grid, tiles);
}

fn generating_adjacency_rules(tiles: &mut [Tile]) {
    let tile_edges: Vec<_> = tiles.iter().map(|tile| tile.edges.clone()).collect();

    for (index, tile) in tiles.iter_mut().enumerate() {
        tile.analyze(&tile_edges, index);
    }
}

fn create_rotate_tiles(tiles: &mut Vec<Tile>) {
    let mut new_tiles = Vec::new();

    for tile in tiles.iter() {
        if tile.is_rotate {
            for j in 1..4 {
                new_tiles.push(tile.rotate(j));
            }
        }
    }
    tiles.extend(new_tiles);
}
