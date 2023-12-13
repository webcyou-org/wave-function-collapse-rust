use sdl2::image::{self, LoadTexture};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

struct Tile {
    texture: Texture,
    edges: Vec<String>,
    angle: f64,
    is_rotate: bool,
}

impl Tile {
    fn load<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        image_path: &str,
        edges: Vec<String>,
        is_rotate: bool,
    ) -> Result<Tile, String> {
        let texture = texture_creator.load_texture(image_path)?;

        Ok(Tile {
            texture,
            edges,
            angle: 0.0,
            is_rotate,
        })
    }
}
