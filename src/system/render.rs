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
    SCREEN_SCALE, // FIXME: Proper handling of window size?
};

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

    fn run(&mut self, (sprite, position, timing): Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 100, 200));
        self.canvas.clear();

        let interp_time = match timing.next_vsync {
            Some(next_vsync) => next_vsync,
            None => Instant::now(),
        };

        let elapsed = interp_time - timing.physics_tick;
        let mut alpha = elapsed.as_secs_f32() / timing.delta_time.as_secs_f32();
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

        // TODO: RenderSystem needs concept of layers or z-index

        for (sprite, position) in (&sprite, &position).join() {
            let sprite_ref = self.sprites.get(sprite.sprite);

            let x = position.x() * alpha + position.previous_x() * (1.0 - alpha);
            let y = position.y() * alpha + position.previous_y() * (1.0 - alpha);

            self.canvas
            .copy(
                sprite_ref.texture(),
                /* FIXME: returning an option here might not be the best idea, since 'None' in this context means "copy the whole source texture" */
                sprite_ref.get_rect_of_frame(sprite.current_frame_idx),
                sdl2::rect::Rect::new(
                    x.round() as i32 - (sprite_ref.frame_width() as u32 / 2 * SCREEN_SCALE) as i32,
                    y.round() as i32 - (sprite_ref.frame_height() as u32 / 2 * SCREEN_SCALE) as i32,
                    sprite_ref.frame_width() as u32 * SCREEN_SCALE,
                    sprite_ref.frame_height() as u32 * SCREEN_SCALE,
                ),
            )
            .unwrap(); // FIXME
        }

        self.canvas.present();
    }
}
