mod constants;
mod core;
mod utility;

use crate::constants::defines::*;
use crate::core::cell::Cell;
use crate::core::tile::Tile;
use crate::utility::error::*;
use crate::utility::utility::{create_tiles_from_json, sdl_init};
use rand::seq::SliceRandom;
use rand::Rng;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use serde::{Deserialize, Serialize};
use std::io::Read;
use tokio;

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_init(&sdl_context)?;
    let texture_creator = canvas.texture_creator();
    let mut tiles = create_tiles_from_json(&texture_creator, JSON_FILE_NAME).await?;
    create_rotate_tiles(&mut tiles);
    generating_adjacency_rules(&mut tiles);

    let mut grid: Vec<Cell> = (0..DIM * DIM)
        .map(|index| Cell::from_value(tiles.len()))
        .collect();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

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
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        draw(&mut canvas, &grid, &tiles);
        canvas.present();
    }

    Ok(())
}

pub fn draw(canvas: &mut Canvas<Window>, grid: &[Cell], tiles: &[Tile]) {
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

pub fn main_loop(grid: &mut Vec<Cell>, tiles: &[Tile]) {
    let mut low_entropy_grid = pick_cell_with_least_entropy(grid);
    if low_entropy_grid.is_empty() {
        return;
    }
    if !random_selection_of_sockets(&mut low_entropy_grid) {
        // start_over(grid);
        println!("start_over");
        return;
    }
    wave_collapse(grid, tiles);
}

pub fn generating_adjacency_rules(tiles: &mut [Tile]) {
    let tile_edges: Vec<_> = tiles.iter().map(|tile| tile.edges.clone()).collect();

    for (index, tile) in tiles.iter_mut().enumerate() {
        tile.analyze(&tile_edges, index);
    }
}

pub fn create_rotate_tiles(tiles: &mut Vec<Tile>) {
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

fn pick_cell_with_least_entropy(grid: &mut Vec<Cell>) -> Vec<&mut Cell> {
    let mut grid_copy: Vec<&mut Cell> = Vec::new();

    for cell in grid.iter_mut() {
        if !cell.collapsed {
            grid_copy.push(cell);
        }
    }
    if grid_copy.is_empty() {
        return Vec::new();
    }
    grid_copy.sort_by_key(|cell| cell.sockets.len());

    let len = grid_copy[0].sockets.len();
    let stop_index = grid_copy
        .iter()
        .position(|cell| cell.sockets.len() > len)
        .unwrap_or(grid_copy.len());

    grid_copy.truncate(stop_index);
    grid_copy
}

fn random_selection_of_sockets(grid_target: &mut Vec<&mut Cell>) -> bool {
    let mut rng = rand::thread_rng();

    if let Some(cell) = grid_target.choose_mut(&mut rng) {
        (*cell).collapsed = true;

        if cell.sockets.is_empty() {
            return false;
        }
        if let Some(&pick) = cell.sockets.choose(&mut rng) {
            cell.sockets = vec![pick];
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn wave_collapse(grid: &mut Vec<Cell>, tiles: &[Tile]) {
    let mut next_grid: Vec<Option<Cell>> = vec![None; DIM * DIM];

    for j in 0..DIM {
        for i in 0..DIM {
            let index = i + j * DIM;

            if grid[index].collapsed {
                next_grid[index] = Some(grid[index].clone());
            } else {
                let mut sockets: Vec<usize> = (0..tiles.len()).collect();
                // Look up
                if j > 0 {
                    cell_collapse(&mut grid[i + (j - 1) * DIM], "down", &mut sockets, tiles);
                }
                // Look right
                if i < DIM - 1 {
                    cell_collapse(&mut grid[i + 1 + j * DIM], "left", &mut sockets, tiles);
                }
                // Look down
                if j < DIM - 1 {
                    cell_collapse(&mut grid[i + (j + 1) * DIM], "up", &mut sockets, tiles);
                }
                // Look left
                if i > 0 {
                    cell_collapse(&mut grid[i - 1 + j * DIM], "right", &mut sockets, tiles);
                }
                next_grid[index] = Some(Cell::from_list(sockets));
            }
        }
    }

    grid.clear();
    grid.extend(next_grid.into_iter().filter_map(|cell| cell));
}

fn cell_collapse(cell: &Cell, direction: &str, sockets: &mut Vec<usize>, tiles: &[Tile]) {
    let valid_sockets = get_valid_sockets(cell, direction, tiles);
    check_valid(sockets, &valid_sockets);
}

fn get_valid_sockets(cell: &Cell, direction: &str, tiles: &[Tile]) -> Vec<usize> {
    let mut valid_sockets = Vec::new();

    for &socket in &cell.sockets {
        let valid = &tiles[socket].valid(direction);
        valid_sockets.extend(valid);
    }

    valid_sockets
}

fn check_valid(sockets: &mut Vec<usize>, valid_sockets: &[usize]) {
    sockets.retain(|socket| valid_sockets.contains(socket));
}
