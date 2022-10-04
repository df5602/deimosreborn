mod errors;

mod sprite;

mod component;
mod entity;
mod resource;
mod system;

use entity::player::Player;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use specs::{DispatcherBuilder, World, WorldExt};

use anyhow::{Context, Result};

use component::{
    player_animation::PlayerAnimationComponent, player_physics::PlayerPhysicsComponent,
    position::PositionComponent, sprite::SpriteComponent,
};
use resource::{player_input::PlayerInput, timing::Timing};
use system::{
    player_animation::PlayerAnimationSystem, player_movement::PlayerMovementSystem,
    render::RenderSystem,
};

use sprite::{Sprite, SpriteDescription, SpriteManager};

use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;
const FRAME_RATE_GAME: u32 = 60;
const FRAME_RATE_RENDER: u32 = 60;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(errors::SdlError::InitError)
        .context("Failed to initialize SDL2")?;

    sdl2::log::set_output_function(|prio, cat, s| {
        println!("SDL2 {:?} [{:?}]: {}", prio, cat, s);
    });

    let video_subsystem = sdl_context
        .video()
        .map_err(errors::SdlError::InitError)
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

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .context("Failed to create canvas")?;

    sdl2::hint::set("SDL_HINT_RENDER_VSYNC", "1");

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

    let mut sprite_manager = SpriteManager::new();
    let player_sprite_id = sprite_manager.insert(player_sprite);

    let mut world = World::new();
    world.insert(PlayerInput::default());
    world.insert(Timing::default());
    world.register::<PlayerAnimationComponent>();
    world.register::<PlayerPhysicsComponent>();
    world.register::<PositionComponent>();
    world.register::<SpriteComponent>();

    {
        let mut timing = world.write_resource::<Timing>();
        timing.delta_time = Duration::from_nanos(1_000_000_000u64 / (FRAME_RATE_GAME as u64));
    }

    Player::create_player(
        &mut world,
        player_sprite_id,
        sprite_manager.get_description(player_sprite_id),
        (SCREEN_WIDTH / 2) as f32,
        (SCREEN_HEIGHT - 200) as f32,
    );

    let mut dispatcher_game = DispatcherBuilder::new()
        .with(PlayerMovementSystem, "player_movement", &[])
        .with(
            PlayerAnimationSystem,
            "player_animation",
            &["player_movement"],
        )
        .build();

    let mut dispatcher_render = DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(canvas, sprite_manager))
        .build();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut next_physics_tick = Instant::now();
    let mut prev_frame_start = Instant::now();

    'running: loop {
        let frame_start = Instant::now();
        println!(
            "INFO [main loop]: Frame start: {:?}, difference to previous: {} us, difference to tick: {} us",
            frame_start,
            (frame_start - prev_frame_start).as_micros(),
            (frame_start - next_physics_tick).as_micros()
        );
        prev_frame_start = frame_start;

        // Run physics update, multiple steps if needed
        loop {
            let now = Instant::now();
            if now > next_physics_tick {
                let elapsed = now - next_physics_tick;

                next_physics_tick +=
                    Duration::from_nanos(1_000_000_000u64 / (FRAME_RATE_GAME as u64));

                // If we missed more than a few updates, take the loss and re-synchronize
                if elapsed > Duration::from_nanos((3 * 1_000_000_000u32 / FRAME_RATE_GAME) as u64) {
                    // Too much time has elapsed, fast-forward
                    continue;
                }

                println!(
                    "INFO [main loop]: Updating physics... elapsed: {} us",
                    elapsed.as_micros()
                );

                {
                    let mut timing = world.write_resource::<Timing>();
                    timing.physics_tick = next_physics_tick;

                    let mut player_input = world.write_resource::<PlayerInput>();

                    for event in event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => break 'running,
                            _ => player_input.update_player_input(event),
                        }
                    }
                }

                dispatcher_game.dispatch(&world);
                world.maintain();
            } else {
                // Not enough time passed, skip..
                break;
            }
        }

        dispatcher_render.dispatch(&world);

        let frame_end = Instant::now();

        // Try to recover vsync
        if (frame_end - frame_start).as_millis() > 8 {
            println!("INFO [main loop]: vsync");
            let mut timing = world.write_resource::<Timing>();
            timing.next_vsync =
                Some(frame_end + Duration::from_nanos(1_000_000_000u64 / FRAME_RATE_RENDER as u64));
        } else {
            let mut timing = world.write_resource::<Timing>();
            if let Some(next_vsync) = timing.next_vsync {
                if frame_end > next_vsync {
                    println!("WARNING [main loop]: reset vsync");
                    timing.next_vsync = None;
                }
            }
        }

        println!(
            "INFO [main loop]: Frame end: {:?}, difference to frame start: {} us",
            frame_end,
            (frame_end - frame_start).as_micros()
        );
    }

    Ok(())
}
