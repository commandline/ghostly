use ai_behavior::Action;
use find_folder;
use gfx_device_gl::Resources;
use piston_window::*;
use ncollide::shape::Cuboid2;
use ncollide::query::{proximity, Proximity};
use nalgebra::{self, Isometry2, Vector2};
use sprite::*;
use uuid::Uuid;
use std::rc::Rc;
use mobs::{wrap, Star};

pub struct Hero {
    pub sprite_id: Uuid,
    pub dir: (f64, f64),
    pub x: f64,
    pub y: f64,
    pub size: u32,
    w: f64,
    h: f64,
    collider: Cuboid2<f64>,
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
        let bounds = sprite.bounding_box();
        let sprite_id = scene.add_child(sprite);
        Hero {
            x: 320.0,
            y: 240.0,
            w: bounds[2],
            h: bounds[3],
            dir: (0.0, 0.0),
            sprite_id: sprite_id,
            size: 2000,
            collider: Cuboid2::new(Vector2::new(bounds[2], bounds[3])),
        }
    }

    pub fn mov(&mut self, w: &PistonWindow, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if let Some(sprite) = scene.child(self.sprite_id) {
            let (sprite_x, sprite_y) = sprite.get_position();
            self.x = sprite_x;
            self.y = sprite_y;
        }
        let (wrapped, new_x, new_y) = wrap((w.size().width.into(), w.size().height.into()),
                                           (self.w, self.h),
                                           (self.x, self.y));
        if wrapped {
            self.x = new_x;
            self.y = new_y;
            if let Some(ref mut sprite) = scene.child_mut(self.sprite_id) {
                sprite.set_position(self.x, self.y);
            }
        }
        let mov_x = self.dir.0 * 2.0;
        let mov_y = self.dir.1 * 2.0;
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::CubicOut,
                               Box::new(MoveBy(dt * 0.75, mov_x, mov_y)))));
    }

    pub fn grow(&mut self, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if let Some(sprite) = scene.child(self.sprite_id) {
            self.size += 60;
            let bounds = sprite.bounding_box();
            self.w = bounds[2];
            self.h = bounds[3];
            self.collider = Cuboid2::new(Vector2::new(bounds[2], bounds[3]));
        }
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::ElasticInOut,
                               Box::new(ScaleBy(dt * 5.0, 0.3, 0.3)))));
    }

    pub fn shrink(&mut self, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if self.size > 0 {
            if let Some(sprite) = scene.child(self.sprite_id) {
                self.size -= 1;
                let bounds = sprite.bounding_box();
                self.w = bounds[2];
                self.h = bounds[3];
                self.collider = Cuboid2::new(Vector2::new(bounds[2], bounds[3]));
            }
            scene.run(self.sprite_id,
                      &Action(Ease(EaseFunction::ElasticInOut,
                                   Box::new(ScaleBy(dt * 5.0, -0.005, -0.005)))));
        }
    }

    pub fn collides(&mut self, star: &Star) -> bool {
        let star_pos = Isometry2::new(Vector2::new(star.x, star.y), nalgebra::zero());
        let pos = Isometry2::new(Vector2::new(self.x, self.y), nalgebra::zero());
        proximity(&star_pos, &star.collider, &pos, &self.collider, 0.0) == Proximity::Intersecting
    }

    pub fn diag(&self) -> String {
        format!("{}: x {} / y {}",
                self.sprite_id,
                self.x.trunc(),
                self.y.trunc())
    }
}
