use crate::utility::error::*;
use crate::utility::utility::compare_edge;
use sdl2::image::{self, LoadTexture};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::f64::consts::PI;
use std::rc::Rc;

pub struct Tile<'a> {
    pub texture: Rc<Texture<'a>>,
    pub edges: Vec<String>,
    pub angle: f64,
    pub is_rotate: bool,
    pub up: Vec<usize>,
    pub right: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
}

impl<'a> Tile<'a> {
    pub fn load(
        texture_creator: &TextureCreator<WindowContext>,
        image_path: String,
        edges: Vec<String>,
        is_rotate: bool,
    ) -> Result<Tile, MyError> {
        let texture = texture_creator.load_texture(format!("assets/images/{}", image_path))?;
        let texture_rc = Rc::new(texture);

        Ok(Tile {
            texture: texture_rc,
            edges,
            angle: 0.0,
            is_rotate,
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
        })
    }

    // pub fn analyze(&mut self, tiles: &[Tile]) {
    //     for (i, tile) in tiles.iter().enumerate() {
    //         // UP
    //         if compare_edge(&tile.edges[2], &self.edges[0]) {
    //             self.up.push(i);
    //         }
    //         // RIGHT
    //         if compare_edge(&tile.edges[3], &self.edges[1]) {
    //             self.right.push(i);
    //         }
    //         // DOWN
    //         if compare_edge(&tile.edges[0], &self.edges[2]) {
    //             self.down.push(i);
    //         }
    //         // LEFT
    //         if compare_edge(&tile.edges[1], &self.edges[3]) {
    //             self.left.push(i);
    //         }
    //     }
    // }

    pub fn analyze(&mut self, tile_edges: &[Vec<String>], current_index: usize) {
        for (index, edges) in tile_edges.iter().enumerate() {
            if index == current_index {
                continue;
            }
            // UP
            if compare_edge(&edges[2], &self.edges[0]) {
                self.up.push(index);
            }
            // RIGHT
            if compare_edge(&edges[3], &self.edges[1]) {
                self.right.push(index);
            }
            // DOWN
            if compare_edge(&edges[0], &self.edges[2]) {
                self.down.push(index);
            }
            // LEFT
            if compare_edge(&edges[1], &self.edges[3]) {
                self.left.push(index);
            }
        }
    }

    pub fn valid(&self, direction: &str) -> Vec<usize> {
        match direction {
            "up" => self.up.clone(),
            "right" => self.right.clone(),
            "down" => self.down.clone(),
            "left" => self.left.clone(),
            _ => Vec::new(),
        }
    }

    pub fn rotate(&self, num: i32) -> Tile<'a> {
        let rotation = num as f64 * (PI / 2.0);
        let new_edges = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let new_index =
                    (i as i32 - num + self.edges.len() as i32) as usize % self.edges.len();
                self.edges[new_index].clone()
            })
            .collect();

        Tile {
            texture: Rc::clone(&self.texture),
            edges: new_edges,
            angle: rotation,
            is_rotate: true,
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
        }
    }
}
