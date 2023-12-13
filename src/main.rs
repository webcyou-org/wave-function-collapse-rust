mod constants;

use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use crate::constants::defines::{GAME_HEIGHT, GAME_WIDTH};

fn sdl_init(sdl_context: &Sdl) -> Result<Canvas<Window>, String>  {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Wave Function Collapse", GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    Ok(canvas)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_init(&sdl_context)?;


    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.present();
    }

    Ok(())
}

