mod errors;
mod sprite;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use anyhow::{Context, Result};
use sprite::Sprite;

use std::time::Duration;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(|e| errors::SdlError::InitError(e))
        .context("Failed to initialize SDL2")?;

    let video_subsystem = sdl_context
        .video()
        .map_err(|e| errors::SdlError::InitError(e))
        .context("Failed to initialize video subsystem")?;

    /*let _image_context = sdl2::image::init(sdl2::image::InitFlag::empty())
    .map_err(|e| errors::SdlError::InitError(e))
    .context("Failed to initialize SDL2_Image")?;*/

    let window = video_subsystem
        .window("Deimos Reborn", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .with_context(|| {
            format!(
                "Could not create main window with dimensions {}x{}",
                SCREEN_WIDTH, SCREEN_HEIGHT
            )
        })?;

    let mut canvas = window
        .into_canvas()
        .build()
        .context("Failed to create canvas")?;

    let texture_creator = canvas.texture_creator();

    let player_sprite = Sprite::from_gif(
        "assets/ Data/Paks/Game/im08/Player 1 Orange IC[pl1o].gif",
        "assets/ Data/Paks/Game/im08/Player 1 Orange IA[PL1O].gif",
        &texture_creator,
    )?;

    canvas.set_draw_color(Color::RGB(0, 100, 200));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

        canvas
            .copy(
                &player_sprite.texture(),
                None,
                sdl2::rect::Rect::new((SCREEN_WIDTH as i32 - (394 * 2)) / 2, 500, 394 * 2, 48 * 2),
            )
            .unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
