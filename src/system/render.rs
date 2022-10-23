use std::time::Instant;

use log::{trace, warn};
use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget},
};
use specs::{Join, Read, ReadStorage, System};

use crate::{
    component::{position::PositionComponent, sprite::SpriteComponent},
    resource::timing::Timing,
    sprite::SpriteManager,
    WINDOW_SCALE, // FIXME: Proper handling of window size?
};

/// Render layer
/// Sprites are rendered according to their associated layer (lower enum value = background)
#[derive(PartialEq, Eq)]
pub enum Layer {
    /// Explosions, bullets, etc.
    Effects,
    /// Player unit, air enemies
    AirUnits,
}

pub struct RenderSystem<'t, T>
where
    T: RenderTarget,
{
    canvas: Canvas<T>,
    sprites: SpriteManager<'t>,
}

impl<'t, T> RenderSystem<'t, T>
where
    T: RenderTarget,
{
    pub fn new(canvas: Canvas<T>, sprite_manager: SpriteManager<'t>) -> Self {
        Self {
            canvas,
            sprites: sprite_manager,
        }
    }

    fn render_layer(
        &mut self,
        system_data: &<RenderSystem<'t, T> as System>::SystemData,
        alpha: f32,
        layer: Layer,
    ) {
        let (sprite, position, _) = system_data;

        for (sprite, position) in (sprite, position)
            .join()
            .filter(|(sprite, _)| sprite.layer == layer)
        {
            let sprite_ref = self.sprites.get(sprite.sprite);

            let x = position.x() * alpha + position.previous_x() * (1.0 - alpha);
            let y = position.y() * alpha + position.previous_y() * (1.0 - alpha);

            self.canvas
            .copy(
                sprite_ref.texture(),
                /* FIXME: returning an option here might not be the best idea, since 'None' in this context means "copy the whole source texture" */
                sprite_ref.get_rect_of_frame(sprite.current_frame_idx),
                sdl2::rect::Rect::new(
                    ((x - sprite_ref.frame_width() as f32 * sprite.scale_factor / 2.0) * WINDOW_SCALE as f32).round() as i32,
                    ((y - sprite_ref.frame_height() as f32 * sprite.scale_factor / 2.0) * WINDOW_SCALE as f32).round() as i32,
                    (sprite_ref.frame_width() as f32 * sprite.scale_factor * WINDOW_SCALE as f32).round() as u32,
                    (sprite_ref.frame_height() as f32 * sprite.scale_factor * WINDOW_SCALE as f32).round() as u32,
                ),
            )
            .unwrap(); // FIXME
        }
    }
}

impl<'sys, 't, T> System<'sys> for RenderSystem<'t, T>
where
    T: RenderTarget,
{
    type SystemData = (
        ReadStorage<'sys, SpriteComponent>,
        ReadStorage<'sys, PositionComponent>,
        Read<'sys, Timing>,
    );

    fn run(&mut self, data: Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 100, 200));
        self.canvas.clear();

        let mut alpha;
        {
            let (_, _, timing) = &data;
            let interp_time = match timing.next_vsync {
                Some(next_vsync) => next_vsync,
                None => Instant::now(),
            };

            let elapsed = interp_time - timing.physics_tick;
            alpha = elapsed.as_secs_f32() / timing.delta_time.as_secs_f32();
            if alpha > 1.0 {
                warn!(target: "RenderSystem", "alpha ({}) > 1.0", alpha);
                alpha = 1.0;
            }

            trace!(target: "RenderSystem",
                "physics tick: {:?}, interp_time: {:?}, diff: {} us, alpha: {}",
                timing.physics_tick,
                interp_time,
                elapsed.as_micros(),
                alpha
            );
        }

        // Render effects
        self.render_layer(&data, alpha, Layer::Effects);

        // Render player, air enemies
        self.render_layer(&data, alpha, Layer::AirUnits);

        self.canvas.present();
    }
}
