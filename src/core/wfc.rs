use crate::constants::defines::DIM;
use crate::core::cell::Cell;
use crate::core::tile::Tile;
use rand::seq::SliceRandom;

pub fn pick_cell_with_least_entropy(grid: &mut Vec<Cell>) -> Vec<&mut Cell> {
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

pub fn random_selection_of_sockets(grid_target: &mut Vec<&mut Cell>) -> bool {
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

pub fn wave_collapse(grid: &mut Vec<Cell>, tiles: &[Tile]) {
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
