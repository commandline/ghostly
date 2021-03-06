use ai_behavior::Action;
use find_folder;
use gfx_device_gl::Resources;
use piston_window::*;
use ncollide::shape::Cuboid2;
use nalgebra::Vector2;
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
    dir: u32,
    pub collider: Cuboid2<f64>,
    pub destroyed: bool,
    accel: f64,
}

const DESIGNED_FOR_WIDTH: f64 = 640.0;

const SCALE_FACTOR: f64 = 0.5;

const MOVE_DUR: f64 = 0.75;
const DESTROY_DUR: f64 = 0.75;

impl Star {
    pub fn new(color: &str, w: &mut PistonWindow, scene: &mut Scene<Texture<Resources>>) -> Star {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tex = Rc::new(Texture::from_path(&mut w.factory,
                                             assets.join(format!("{}_star.png", color)),
                                             Flip::None,
                                             &TextureSettings::new())
            .unwrap());
        let Size { width, height } = w.size();
        let x = rand_pos(width as f64);
        let y = rand_pos(height as f64);
        let scale = width as f64 / DESIGNED_FOR_WIDTH * SCALE_FACTOR;
        let accel = match color {
            "yellow" => 3.0,
            "blue" => 1.0,
            "green" => 2.0,
            _ => 2.0,
        };
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_scale(scale, scale);
        sprite.set_position(x, y);
        let bounds = sprite.bounding_box();
        let sprite_id = scene.add_child(sprite);
        Star {
            x: x,
            y: y,
            w: bounds[2],
            h: bounds[3],
            sprite_id: sprite_id,
            dir: rand_dir(),
            collider: Cuboid2::new(Vector2::new(bounds[2], bounds[3])),
            destroyed: false,
            accel: accel,
        }
    }

    pub fn mov(&mut self, w: &PistonWindow, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        if self.destroyed {
            return;
        }
        if let Some(sprite) = scene.child(self.sprite_id) {
            let (sprite_x, sprite_y) = sprite.get_position();
            self.x = sprite_x;
            self.y = sprite_y;
        }
        let (wrapped, new_x, new_y) = wrap((w.size().width.into(), w.size().height.into()),
                                           (self.w, self.h),
                                           (self.x, self.y));
        if wrapped {
            if let Some(ref mut sprite) = scene.child_mut(self.sprite_id) {
                self.x = new_x;
                self.y = new_y;
                sprite.set_position(self.x, self.y);
            }
        }
        self.dir = rand_turn(self.dir);
        let dir = lookup_dir(self.dir);
        let mov_x = self.accel * dir.0;
        let mov_y = self.accel * dir.1;
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::CubicOut,
                               Box::new(MoveBy(dt * MOVE_DUR, mov_x, mov_y)))));
    }

    pub fn destroy(&mut self, scene: &mut Scene<Texture<Resources>>, dt: f64) {
        scene.run(self.sprite_id,
                  &Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(dt * DESTROY_DUR)))));
        self.destroyed = true;
    }
}

fn rand_dir() -> u32 {
    rand::random::<u32>() % 8
}

fn rand_pos(dim: f64) -> f64 {
    (rand::random::<f64>() % dim).abs()
}

fn rand_turn(dir: u32) -> u32 {
    let coin = rand::random::<i32>() % 10;
    match coin {
        -9 => if dir == 0 { 7 } else { dir - 1 },
        9 => if dir == 7 { 0 } else { dir + 1 },
        _ => dir,
    }
}

fn lookup_dir(dir: u32) -> (f64, f64) {
    match dir {
        0 => (0.0, -1.0),
        1 => (1.0, -1.0),
        2 => (1.0, 0.0),
        3 => (1.0, 1.0),
        4 => (0.0, 1.0),
        5 => (-1.0, 1.0),
        6 => (-1.0, 0.0),
        7 => (-1.0, -1.0),
        _ => (0.0, 0.0),
    }
}
