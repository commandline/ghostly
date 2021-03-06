extern crate ai_behavior;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate nalgebra;
extern crate ncollide;
extern crate piston_window;
extern crate rand;
extern crate sprite;
extern crate uuid;

mod mobs;

use gfx_device_gl::Resources;
use graphics::Image;
use graphics::rectangle::square;
use piston_window::*;
use mobs::{Hero, Star};
use sprite::*;

pub struct Game {
    scene: Scene<Texture<Resources>>,
    player: Hero,
    stars: Vec<Star>,
    diag: bool,
    pause: bool,
    victory: bool,
    loss: bool,
}

const TILE_SIZE: u32 = 64;

impl Game {
    pub fn new(w: &mut PistonWindow) -> Game {
        let mut scene = Scene::new();
        let mut stars: Vec<Star> = vec![];
        for number in 1..7 {
            let color = match number % 4 {
                1 => "yellow",
                2 => "green",
                3 => "blue",
                _ => "pink",
            };
            stars.push(Star::new(color, w, &mut scene));
        }
        let player = Hero::new(w, &mut scene);
        Game {
            scene: scene,
            player: player,
            stars: stars,
            diag: false,
            pause: false,
            victory: false,
            loss: false,
        }
    }

    pub fn on_update(&mut self, e: &Input, upd: UpdateArgs, w: &PistonWindow) {
        if self.pause || self.victory || self.loss {
            return;
        }
        self.scene.event(e);

        let mut grew = false;
        for mut star in &mut self.stars {
            if !star.destroyed && self.player.collides(&star) {
                star.destroy(&mut self.scene, upd.dt);
                self.player.grow(&mut self.scene, upd.dt);
                grew = true;
            } else {
                star.mov(w, &mut self.scene, upd.dt);
            }
        }

        if self.stars.iter().all(|star| star.destroyed) {
            self.victory = true;
            self.loss = false;
            return;
        }

        if !grew {
            self.player.shrink(&mut self.scene, upd.dt);
        }
        if self.player.size > 0.0 {
            self.player.mov(w, &mut self.scene, upd.dt);
        } else  {
            self.loss = true;
            self.victory = false;
        }
    }

    pub fn on_draw(&mut self, e: &Input, _: RenderArgs, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("Fresca-Regular.ttf");
        let factory = w.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

        let Size { height, width } = w.size();
        let image = Image::new().rect(square(0.0, 0.0, width as f64));
        let bg = Texture::from_path(&mut w.factory,
                                       assets.join("bg.png"),
                                       Flip::None,
                                       &TextureSettings::new()).unwrap();
        w.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            let cols = width / TILE_SIZE;
            // 4:3 means rows will be fractional so add one to cover completely
            let tile_count = cols * (height / TILE_SIZE + 1);
            for number in 0..tile_count {
                let x: f64 = (number % cols * TILE_SIZE).into();
                let y: f64 = (number / cols * TILE_SIZE).into();
                image.draw(&bg, &Default::default(), c.transform.trans(x, y).zoom(1.0 / cols as f64), g);
            }

            if self.victory {
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], height / 10).draw(
                    "Hurray! You win!",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(width as f64 / 2.0 - width as f64 / 4.5, height as f64 / 2.0), g
                );
                return;
            } else if self.loss {
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], height / 10).draw(
                    "Aw! You lose!",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(width as f64 / 2.0 - width as f64 / 4.5, height as f64 / 2.0), g
                );
                return;
            }

            if self.diag {
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 10).draw(
                    &format!("{}", self.player.diag()),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 10.0), g
                );
            }

            self.scene.draw(c.transform, g);
        });
    }

    // TODO use an enum to track requested movement direction
    pub fn on_input(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.player.dir = (self.player.dir.0, -1.0);
                    }
                    Button::Keyboard(Key::Down) => {
                        self.player.dir = (self.player.dir.0, 1.0);
                    }
                    Button::Keyboard(Key::Left) => {
                        self.player.dir = (-1.0, self.player.dir.1);
                    }
                    Button::Keyboard(Key::Right) => {
                        self.player.dir = (1.0, self.player.dir.1);
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.player.dir = (self.player.dir.0, 0.0);
                    }
                    Button::Keyboard(Key::Down) => {
                        self.player.dir = (self.player.dir.0, 0.0);
                    }
                    Button::Keyboard(Key::Left) => {
                        self.player.dir = (0.0, self.player.dir.1);
                    }
                    Button::Keyboard(Key::Right) => {
                        self.player.dir = (0.0, self.player.dir.1);
                    }
                    Button::Keyboard(Key::H) => {
                        self.diag = !self.diag;
                    }
                    Button::Keyboard(Key::P) => {
                        self.pause = !self.pause;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
