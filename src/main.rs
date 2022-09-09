mod errors;
mod sprite;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use anyhow::{Context, Result};
use sprite::{Sprite, SpriteDescription};

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
        SpriteDescription {
            number_of_frames: 7,
            border_left: 3,
            border_up: 3,
            frame_dimensions: (53, 43),
        },
        "assets/ Data/Paks/Game/im08/Player 1 Orange IC[pl1o].gif",
        "assets/ Data/Paks/Game/im08/Player 1 Orange IA[PL1O].gif",
        &texture_creator,
    )?;

    canvas.set_draw_color(Color::RGB(0, 100, 200));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let frames = [0, 1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4];
    let mut frame_count = 0;
    let mut frame_idx = 0;

    'running: loop {
        canvas.clear();

        canvas
            .copy(
                &player_sprite.texture(),
                /* FIXME: returning an option here might not be the best idea, since 'None' in this context means "copy the whole source texture" */
                player_sprite.get_rect_of_frame(frames[frame_idx]),
                sdl2::rect::Rect::new(
                    (SCREEN_WIDTH as i32 - (player_sprite.frame_width() * 2) as i32) / 2,
                    500,
                    (player_sprite.frame_width() * 2) as u32,
                    (player_sprite.frame_height() * 2) as u32,
                ),
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

        frame_count += 1;
        frame_idx = frame_count / 6;

        if frame_idx >= frames.len() {
            frame_idx = 0;
            frame_count = 0;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
