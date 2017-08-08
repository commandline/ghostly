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
    pub size: f64,
    w: f64,
    h: f64,
    collider: Cuboid2<f64>,
}

const DESIGNED_FOR_WIDTH: f64 = 640.0;

const BASE_SIZE: f64 = 500.0;
const GROWTH_FACTOR: f64 = 50.0;
const SHRINK_FACTOR: f64 = 0.5;

const GROW_SHRINK_DUR: f64 = 5.0;
const MOVE_DUR: f64 = 0.75;

const ACCEL: f64 = 2.0;

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
        let Size { width, height } = w.size();
        let x = width as f64 / 2.0;
        let y = height as f64 / 2.0;
        let scale = width as f64 / DESIGNED_FOR_WIDTH;
        sprite.set_position(x, y);
        sprite.set_scale(scale, scale);
        let bounds = sprite.bounding_box();
        let sprite_id = scene.add_child(sprite);
        Hero {
            x: x,
            y: y,
            w: bounds[2],
            h: bounds[3],
            dir: (0.0, 0.0),
            sprite_id: sprite_id,
            size: BASE_SIZE,
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
        let mov_x = self.dir.0 * ACCEL;
        let mov_y = self.dir.1 * ACCEL;
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::CubicOut,
                               Box::new(MoveBy(dt * MOVE_DUR, mov_x, mov_y)))));
    }

    pub fn grow(&mut self, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        self.size += GROWTH_FACTOR;
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::ElasticInOut,
                               Box::new(ScaleTo(dt * GROW_SHRINK_DUR, self.size / BASE_SIZE, self.size / BASE_SIZE)))));
        if let Some(sprite) = scene.child(self.sprite_id) {
            let bounds = sprite.bounding_box();
            self.w = bounds[2];
            self.h = bounds[3];
            self.collider = Cuboid2::new(Vector2::new(bounds[2], bounds[3]));
        }
    }

    pub fn shrink(&mut self, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if self.size > 0.0 {
                self.size -= SHRINK_FACTOR;
            scene.run(self.sprite_id,
                      &Action(Ease(EaseFunction::ElasticInOut,
                                   Box::new(ScaleTo(dt * GROW_SHRINK_DUR, self.size / BASE_SIZE, self.size / BASE_SIZE)))));
            if let Some(sprite) = scene.child(self.sprite_id) {
                let bounds = sprite.bounding_box();
                self.w = bounds[2];
                self.h = bounds[3];
                self.collider = Cuboid2::new(Vector2::new(bounds[2], bounds[3]));
            }
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
