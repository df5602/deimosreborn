mod errors;

mod sound;
mod sprite;

mod component;
mod entity;
mod resource;
mod system;

use log::{debug, error, info, trace, warn};
use simple_logger::SimpleLogger;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use specs::{DispatcherBuilder, World, WorldExt};

use anyhow::{Context, Result};

use component::{
    bullet_physics::BulletPhysicsComponent, player_animation::PlayerAnimationComponent,
    player_physics::PlayerPhysicsComponent, player_weapon::PlayerWeaponComponent,
    position::PositionComponent, sprite::SpriteComponent,
};
use entity::player::Player;
use resource::{player_input::PlayerInput, sound::SoundSystem, timing::Timing};
use system::bullet_physics::BulletPhysicsSystem;
use system::{
    player_animation::PlayerAnimationSystem, player_movement::PlayerMovementSystem,
    player_weapon::PlayerWeaponSystem, render::RenderSystem,
};

use sound::{SoundId, SoundManager};
use sprite::{Sprite, SpriteDescription, SpriteManager};

use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const SCREEN_SCALE: u32 = 2;
const SCREEN_WIDTH: u32 = 640 * SCREEN_SCALE;
const SCREEN_HEIGHT: u32 = 480 * SCREEN_SCALE;
const FRAME_RATE_GAME: u32 = 60;
const FRAME_RATE_RENDER: u32 = 60;

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .env()
        .init()?;

    let sdl_context = sdl2::init()
        .map_err(errors::SdlError::InitError)
        .context("Failed to initialize SDL2")?;

    sdl2::log::set_output_function(|prio, cat, s| match prio {
        sdl2::log::Priority::Verbose => trace!(target: &format!("SDL2 [{:?}]", cat), "{}", s),
        sdl2::log::Priority::Debug => debug!(target: &format!("SDL2 [{:?}]", cat), "{}", s),
        sdl2::log::Priority::Info => info!(target: &format!("SDL2 [{:?}]", cat), "{}", s),
        sdl2::log::Priority::Warn => warn!(target: &format!("SDL2 [{:?}]", cat), "{}", s),
        sdl2::log::Priority::Error | sdl2::log::Priority::Critical => {
            error!(target: &format!("SDL2 [{:?}]", cat), "{}", s)
        }
    });

    let video_subsystem = sdl_context
        .video()
        .map_err(errors::SdlError::InitError)
        .context("Failed to initialize video subsystem")?;

    /*let _image_context = sdl2::image::init(sdl2::image::InitFlag::empty())
    .map_err(errors::SdlError::InitError)
    .context("Failed to initialize SDL2_Image")?;*/

    /*let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::empty())
    .map_err(errors::SdlError::InitError)
    .context("Failed to initialize SDL2_mixer")?;*/

    sdl2::mixer::open_audio(44100, sdl2_sys::mixer::MIX_DEFAULT_FORMAT as u16, 2, 1024)
        .map_err(errors::SdlError::InitError)
        .context("Failed to open audio driver")?;

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
    let mut sprite_manager = SpriteManager::new();

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
    let player_sprite_id = sprite_manager.insert(player_sprite);

    let ion_cannon_bullet_sprite = Sprite::from_gif(
        SpriteDescription {
            number_of_frames: 36,
            border_left: 3,
            border_up: 3,
            frame_dimensions: (20, 20),
        },
        "assets/ Data/Paks/Game/im08/Ion Cannon Bullet IC[icbu].gif",
        "assets/ Data/Paks/Game/im08/Ion Cannon Bullet IA[ICBU].gif",
        &texture_creator,
    )?;

    let bullet_sprite_id = sprite_manager.insert(ion_cannon_bullet_sprite);

    let mut sound_manager = SoundManager::new();

    let sound_ion_cannon_bullet =
        sdl2::mixer::Chunk::from_file("assets/ Data/Paks/Audio/Ion-Cannon-Bullet_icbu_.wav")
            .map_err(errors::SdlError::SoundLoadError)?;

    let bullet_sound_id = sound_manager.insert(sound_ion_cannon_bullet);

    let audio_channel = sdl2::mixer::Channel::all();
    let (audio_sender, audio_receiver) = channel::<SoundId>();

    let mut world = World::new();
    world.insert(PlayerInput::default());
    world.insert(Timing::default());
    world.insert(SoundSystem {
        sender: Mutex::new(audio_sender),
    });
    world.register::<BulletPhysicsComponent>();
    world.register::<PlayerAnimationComponent>();
    world.register::<PlayerPhysicsComponent>();
    world.register::<PlayerWeaponComponent>();
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
        bullet_sprite_id,
        bullet_sound_id,
    );

    let mut dispatcher_game = DispatcherBuilder::new()
        .with(PlayerMovementSystem, "player_movement", &[])
        .with(PlayerWeaponSystem, "player_weapon", &["player_movement"])
        .with(BulletPhysicsSystem, "bullet_physics", &[])
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
        trace!(target: "main loop",
            "Frame start: {:?}, difference to previous: {} us, difference to tick: {} us",
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
                    warn!(target: "main loop", "Missed too many physics updates, elapsed: {} us", elapsed.as_micros());
                    continue;
                }

                trace!(target: "main loop",
                    "Updating physics... elapsed: {} us",
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

        // Sounds
        for sound in audio_receiver.try_iter() {
            audio_channel
                .play(sound_manager.get(sound), 0)
                .map_err(errors::SdlError::AudioPlayError)?;
        }

        // Render
        dispatcher_render.dispatch(&world);

        let frame_end = Instant::now();

        // Try to recover vsync
        if (frame_end - frame_start).as_millis() > 8 {
            let next_vsync =
                frame_end + Duration::from_nanos(1_000_000_000u64 / FRAME_RATE_RENDER as u64);
            trace!(target: "main loop", "next vsync: {:?}", next_vsync);
            let mut timing = world.write_resource::<Timing>();
            timing.next_vsync = Some(next_vsync);
        } else {
            let mut timing = world.write_resource::<Timing>();
            if let Some(next_vsync) = timing.next_vsync {
                if frame_end > next_vsync {
                    warn!(target: "main loop", "reset vsync");
                    timing.next_vsync = None;
                }
            }
        }

        trace!(target: "main loop",
            "Frame end: {:?}, difference to frame start: {} us",
            frame_end,
            (frame_end - frame_start).as_micros()
        );
    }

    Ok(())
}
