use ai_behavior::Action;
use find_folder;
use gfx_device_gl::Resources;
use piston_window::*;
use sprite::*;
use uuid::Uuid;
use rand;
use std::rc::Rc;
use super::wrap;


pub struct Star {
    pub sprite_id: Uuid,
    pub x: f64,
    pub y: f64,
    w: f64,
    h: f64,
}

impl Star {
    pub fn new(w: &mut PistonWindow, scene: &mut Scene<Texture<Resources>>) -> Star {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tex = Rc::new(Texture::from_path(&mut w.factory,
                                             assets.join("star.png"),
                                             Flip::None,
                                             &TextureSettings::new())
                                  .unwrap());
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_scale(0.5, 0.5);
        sprite.set_position(30.0, 32.0);
        let bounds = sprite.bounding_box();
        let sprite_id = scene.add_child(sprite);
        Star {
            x: 32.0,
            y: 32.0,
            w: bounds[2],
            h: bounds[3],
            sprite_id: sprite_id,
        }
    }

    pub fn mov(&mut self, w: &PistonWindow, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if let Some(sprite) = scene.child(self.sprite_id) {
            let (sprite_x, sprite_y) = sprite.get_position();
            self.x = sprite_x;
            self.y = sprite_y;
        }
        let (wrapped, new_x, new_y) = wrap((w.size().width.into(), w.size().height.into()), (self.w, self.h), (self.x, self.y));
        if wrapped {
            if let Some(ref mut sprite) = scene.child_mut(self.sprite_id) {
                self.x = new_x;
                self.y = new_y;
                sprite.set_position(self.x, self.y);
            }
        }
        let mov_x = rand::random::<f64>() * 0.5;
        let mov_y = rand::random::<f64>() * 0.5;
        scene.run(self.sprite_id, &Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(dt * 0.75, mov_x, mov_y)))));
    }
}