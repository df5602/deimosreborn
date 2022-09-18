use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget},
};
use specs::{Join, ReadStorage, System};

use crate::{
    component::{
        player_animation::PlayerAnimationComponent, position::PositionComponent,
        sprite::SpriteComponent,
    },
    sprite::SpriteManager,
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
        ReadStorage<'sys, PlayerAnimationComponent>,
    );

    fn run(&mut self, (sprite, position, animation): Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 100, 200));
        self.canvas.clear();

        for (sprite, position, animation) in (&sprite, &position, &animation).join() {
            let sprite_ref = self.sprites.get(sprite.sprite);
            self.canvas
            .copy(
                sprite_ref.texture(),
                /* FIXME: returning an option here might not be the best idea, since 'None' in this context means "copy the whole source texture" */
                sprite_ref.get_rect_of_frame(animation.sprite_frame_idx),
                sdl2::rect::Rect::new(
                    position.x as i32 - (sprite_ref.frame_width()) as i32,
                    position.y as i32 - (sprite_ref.frame_height()) as i32,
                    (sprite_ref.frame_width() * 2) as u32,
                    (sprite_ref.frame_height() * 2) as u32,
                ),
            )
            .unwrap(); // FIXME
        }

        self.canvas.present();
    }
}
