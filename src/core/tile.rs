use crate::utility::error::*;
use sdl2::image::{self, LoadTexture};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Tile<'a> {
    texture: Texture<'a>,
    edges: Vec<String>,
    angle: f64,
    is_rotate: bool,
}

impl<'a> Tile<'a> {
    pub fn load(
        texture_creator: &'a TextureCreator<WindowContext>,
        image_path: String,
        edges: Vec<String>,
        is_rotate: bool,
    ) -> Result<Tile<'a>, MyError> {
        let texture = texture_creator.load_texture(image_path)?;

        Ok(Tile {
            texture,
            edges,
            angle: 0.0,
            is_rotate,
        })
    }
}
