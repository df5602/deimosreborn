use std::path::Path;

use sdl2::image::ImageRWops;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;

use crate::errors::SdlError;
use anyhow::{bail, Result};

#[derive(Debug)]
pub struct SpriteDescription {
    pub number_of_frames: usize,
    pub border_left: usize,
    pub border_up: usize,
    pub frame_dimensions: (usize, usize),
}

pub struct Sprite<'t> {
    texture: Texture<'t>,
    description: SpriteDescription,
}

impl<'t> Sprite<'t> {
    pub fn from_gif<P: AsRef<Path>, T>(
        description: SpriteDescription,
        path_color_map: P,
        path_alpha_map: P,
        texture_creator: &'t TextureCreator<T>,
    ) -> Result<Sprite<'t>> {
        let color_map = Self::load_sprite_from_gif(path_color_map)?;
        let alpha_map = Self::load_sprite_from_gif(path_alpha_map)?;

        let texture = Self::create_texture(&color_map, &alpha_map, texture_creator)?;
        let texture_query = texture.query();

        if (description.border_left + description.frame_dimensions.0) * description.number_of_frames
            > texture_query.width as usize
        {
            // TODO: add context (e.g. sprite name)
            bail!("Width according to sprite description exceeds texture width [sprite description: {:#?}, texture properties: {:#?}", description, texture_query);
        }

        if description.border_up + description.frame_dimensions.1 > texture_query.height as usize {
            // TODO: add context (e.g. sprite name)
            bail!("Height according to sprite description exceeds texture height [sprite description: {:#?}, texture properties: {:#?}", description, texture_query);
        }

        Ok(Self {
            texture,
            description,
        })
    }

    pub fn texture(&self) -> &Texture<'t> {
        &self.texture
    }

    pub fn frame_width(&self) -> usize {
        self.description.frame_dimensions.0
    }

    pub fn frame_height(&self) -> usize {
        self.description.frame_dimensions.1
    }

    pub fn get_rect_of_frame(&self, frame: usize) -> Option<Rect> {
        if frame < self.description.number_of_frames {
            Some(Rect::new(
                (frame * self.description.frame_dimensions.0
                    + (frame + 1) * self.description.border_left) as i32,
                self.description.border_up as i32,
                self.description.frame_dimensions.0 as u32,
                self.description.frame_dimensions.1 as u32,
            ))
        } else {
            None
        }
    }

    fn load_sprite_from_gif<P>(path: P) -> Result<Surface<'static>, SdlError>
    where
        P: AsRef<Path>,
    {
        let rwops = RWops::from_file(path, "rb").map_err(SdlError::SpriteLoadError)?;
        rwops.load_gif().map_err(SdlError::SpriteLoadError)
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

        let mut texture = target_surface.as_texture(texture_creator)?;
        texture.set_blend_mode(BlendMode::Blend);

        Ok(texture)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpriteId(usize);

pub struct SpriteManager<'t> {
    sprites: Vec<Sprite<'t>>,
}

impl<'t> SpriteManager<'t> {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    pub fn insert(&mut self, sprite: Sprite<'t>) -> SpriteId {
        self.sprites.push(sprite);
        SpriteId(self.sprites.len() - 1)
    }

    pub fn get(&self, id: SpriteId) -> &Sprite<'t> {
        &self.sprites[id.0]
    }
}
