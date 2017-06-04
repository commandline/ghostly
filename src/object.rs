use ai_behavior::Action;
use find_folder;
use gfx_device_gl::Resources;
use piston_window::*;
use sprite::*;
use uuid::Uuid;
use std::rc::Rc;


pub struct Object {
    sprite_id: Uuid,
    x: f64,
    y: f64,
}

impl Object {
    pub fn new(w: &mut PistonWindow, scene: &mut Scene<Texture<Resources>>) -> Object {
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
        Object {
            x: 0.0,
            y: 0.0,
            sprite_id: sprite_id,
        }
    }

    pub fn mov(&mut self, scene: &mut Scene<Texture<Resources>>, x: f64, y: f64) {
        self.x += x;
        self.y += y;
        let mov = Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, x, y))));
        scene.run(self.sprite_id, &mov);
    }
}
