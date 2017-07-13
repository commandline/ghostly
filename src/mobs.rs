use ai_behavior::Action;
use find_folder;
use gfx_device_gl::Resources;
use piston_window::*;
use sprite::*;
use uuid::Uuid;
use std::rc::Rc;


// TODO add size of player sprite for boundary checking
pub struct Hero {
    pub sprite_id: Uuid,
    pub x: f64,
    pub y: f64,
}

impl Hero {
    pub fn new(w: &mut PistonWindow, scene: &mut Scene<Texture<Resources>>) -> Hero {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tex = Rc::new(Texture::from_path(&mut w.factory,
                                             assets.join("soot.png"),
                                             Flip::None,
                                             &TextureSettings::new())
                                  .unwrap());
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_position(320.0, 240.0);
        let sprite_id = scene.add_child(sprite);
        Hero {
            x: 320.0,
            y: 240.0,
            sprite_id: sprite_id,
        }
    }

    pub fn mov(&mut self, scene: &mut Scene<Texture<Resources>>, input_x: f64, input_y: f64) {
        if let Some(sprite) = scene.child(self.sprite_id) {
            let (sprite_x, sprite_y) = sprite.get_position();
            self.x = sprite_x;
            self.y = sprite_y;
        }
        let mut wrapped = false;
        // TODO pass in window size
        if self.x > 640.0 + 32.0 {
            self.x = -32.0;
            wrapped = true;
        } else if self.x < -32.0 {
            self.x = 640.0 + 32.0;
            wrapped = true;
        }
        if self.y > 480.0 + 32.0 {
            self.y = -32.0;
            wrapped = true;
        } else if self.y < -32.0 {
            self.y = 480.0 + 32.0;
            wrapped = true;
        }
        self.y += input_y;
        if wrapped {
            scene.stop_all(self.sprite_id);
            if let Some(ref mut sprite) = scene.child_mut(self.sprite_id) {
                sprite.set_position(self.x, self.y);
            }
        }
        scene.run(self.sprite_id, &Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, input_x, input_y)))));
    }
}
