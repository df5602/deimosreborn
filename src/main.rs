mod errors;

mod sprite;

mod component;
mod resource;
mod system;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use specs::{Builder, DispatcherBuilder, World, WorldExt};

use anyhow::{Context, Result};

use component::{
    player_animation::PlayerAnimationComponent, player_physics::PlayerPhysicsComponent,
    position::PositionComponent, sprite::SpriteComponent,
};
use resource::player_input::PlayerInput;
use system::{
    player_animation::PlayerAnimationSystem, player_movement::PlayerMovementSystem,
    render::RenderSystem,
};

use sprite::{Sprite, SpriteDescription, SpriteManager};

use std::time::Duration;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(errors::SdlError::InitError)
        .context("Failed to initialize SDL2")?;

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

    let mut sprite_manager = SpriteManager::new();
    let player_sprite_id = sprite_manager.insert(player_sprite);

    let mut world = World::new();
    world.insert(PlayerInput::default());
    world.register::<PlayerAnimationComponent>();
    world.register::<PlayerPhysicsComponent>();
    world.register::<PositionComponent>();
    world.register::<SpriteComponent>();

    world
        .create_entity()
        .with(SpriteComponent {
            sprite: player_sprite_id,
        })
        .with(PositionComponent {
            x: (SCREEN_WIDTH / 2) as f32,
            y: (SCREEN_HEIGHT - 200) as f32,
        })
        .with(PlayerPhysicsComponent {
            ax: 0.0,
            ay: 0.0,
            vx: 0.0,
            vy: 0.0,
            ax_max: 1.0, // FIXME: insert real values
            ay_max: 1.0,
            vx_max: 8.0,
            vy_max: 8.0,
            x_min: (50 + 53/* FIXME */) as f32,
            x_max: (SCREEN_WIDTH - 50 - 53/* FIXME */) as f32,
            y_min: (50 + 43/* FIXME */) as f32,
            y_max: (SCREEN_HEIGHT - 50 - 43/* FIXME */) as f32,
        })
        .with(PlayerAnimationComponent::default())
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerMovementSystem, "player_movement", &[])
        .with(
            PlayerAnimationSystem,
            "player_animation",
            &["player_movement"],
        )
        .with_thread_local(RenderSystem::new(canvas, sprite_manager))
        .build();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        {
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

        dispatcher.dispatch(&world);
        world.maintain();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
