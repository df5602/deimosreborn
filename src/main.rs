use sdl2::event::Event;
use sdl2::image::ImageRWops;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;

fn create_texture<'t, T>(
    color_map: &Surface,
    alpha_map: &Surface,
    texture_creator: &'t TextureCreator<T>,
) -> Texture<'t> {
    assert_eq!(color_map.pixel_format_enum(), PixelFormatEnum::RGB888);
    assert_eq!(alpha_map.pixel_format_enum(), PixelFormatEnum::RGB888);
    assert_eq!(color_map.size(), alpha_map.size());

    let pixels_alpha = alpha_map.without_lock().unwrap();
    let mut target_surface = color_map.convert_format(PixelFormatEnum::BGRA8888).unwrap();
    let pixels_target = target_surface.without_lock_mut().unwrap();

    for pixel in pixels_target.chunks(4).skip(394 * 3).take(50) {
        println!("{:?}", pixel);
    }

    for (i, pixel) in pixels_alpha.chunks(4).enumerate() {
        let grayscale = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
        //println!("{:?} -> {}", pixel, grayscale);
        pixels_target[i * 4] = 255 - grayscale;
    }

    for pixel in pixels_target.chunks(4).skip(394 * 3).take(50) {
        println!("{:?}", pixel);
    }

    let mut texture = target_surface.as_texture(&texture_creator).unwrap();
    texture.set_blend_mode(BlendMode::Blend);
    return texture;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG).unwrap();

    let window = video_subsystem
        .window("Deimos Reborn", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let rwops_color = RWops::from_file(
        "assets/ Data/Paks/Game/im08/Player 1 Orange IC[pl1o].gif",
        "rb",
    )
    .unwrap();
    let surface_color = rwops_color.load_gif().unwrap();
    let texture_color = surface_color.as_texture(&texture_creator).unwrap();
    println!(
        "Color sprite (surface): format: {:?}, must lock: {}",
        surface_color.pixel_format_enum(),
        surface_color.must_lock()
    );
    println!("Color sprite (texture): {:#?}", texture_color.query());

    let rwops_mask = RWops::from_file(
        "assets/ Data/Paks/Game/im08/Player 1 Orange IA[PL1O].gif",
        "rb",
    )
    .unwrap();
    let surface_mask = rwops_mask.load_gif().unwrap();
    let texture_mask = surface_mask.as_texture(&texture_creator).unwrap();
    println!(
        "Mask sprite (surface): format: {:?}",
        surface_mask.pixel_format_enum()
    );
    println!("Mask sprite (texture): {:#?}", texture_mask.query());

    let texture_final = create_texture(&surface_color, &surface_mask, &texture_creator);

    canvas.set_draw_color(Color::RGB(0, 100, 200));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        canvas
            .copy(
                &texture_color,
                None,
                sdl2::rect::Rect::new((SCREEN_WIDTH as i32 - (394 * 2)) / 2, 100, 394 * 2, 48 * 2),
            )
            .unwrap();
        canvas
            .copy(
                &texture_mask,
                None,
                sdl2::rect::Rect::new((SCREEN_WIDTH as i32 - (394 * 2)) / 2, 300, 394 * 2, 48 * 2),
            )
            .unwrap();
        canvas
            .copy(
                &texture_final,
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
}
