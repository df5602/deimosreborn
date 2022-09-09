use std::path::Path;

use sdl2::image::ImageRWops;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;

use crate::errors::SdlError;
use anyhow::Result;

pub struct Sprite<'t> {
    texture: Texture<'t>,
}

impl<'t> Sprite<'t> {
    pub fn from_gif<P: AsRef<Path>, T>(
        path_color_map: P,
        path_alpha_map: P,
        texture_creator: &'t TextureCreator<T>,
    ) -> Result<Sprite<'t>> {
        let color_map = Self::load_sprite_from_gif(path_color_map)?;
        let alpha_map = Self::load_sprite_from_gif(path_alpha_map)?;

        Ok(Self {
            texture: Self::create_texture(&color_map, &alpha_map, &texture_creator)?,
        })
    }

    pub fn texture(&self) -> &Texture<'t> {
        &self.texture
    }

    fn load_sprite_from_gif<P>(path: P) -> Result<Surface<'static>, SdlError>
    where
        P: AsRef<Path>,
    {
        let rwops = RWops::from_file(path, "rb").map_err(|e| SdlError::SpriteLoadError(e))?;
        rwops.load_gif().map_err(|e| SdlError::SpriteLoadError(e))
    }

    fn create_texture<T>(
        color_map: &Surface,
        alpha_map: &Surface,
        texture_creator: &'t TextureCreator<T>,
    ) -> Result<Texture<'t>> {
        assert_eq!(color_map.pixel_format_enum(), PixelFormatEnum::RGB888);
        assert_eq!(alpha_map.pixel_format_enum(), PixelFormatEnum::RGB888);
        assert_eq!(color_map.size(), alpha_map.size());

        let pixels_alpha = alpha_map
            .without_lock()
            .expect("surface doesn't require locking");
        let mut target_surface = color_map
            .convert_format(PixelFormatEnum::BGRA8888)
            .map_err(|e| {
                SdlError::SpriteLoadError(format!("Could not create BGRA8888 surface: {}", e))
            })?;
        let pixels_target = target_surface
            .without_lock_mut()
            .expect("surface doesn't require locking");

        for (i, pixel) in pixels_alpha.chunks(4).enumerate() {
            let grayscale = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
            pixels_target[i * 4] = 255 - grayscale;
        }

        let mut texture = target_surface.as_texture(&texture_creator)?;
        texture.set_blend_mode(BlendMode::Blend);

        Ok(texture)
    }
}
