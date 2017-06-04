extern crate ai_behavior;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

mod object;

use gfx_device_gl::Resources;
use piston_window::*;
use object::Object;
use sprite::*;

pub struct Game {
    scene: Scene<Texture<Resources>>,
    player: Object,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}

impl Game {
    pub fn new(w: &mut PistonWindow) -> Game {
        let mut scene = Scene::new();
        let player = Object::new(w, &mut scene);
        Game {
            scene: scene,
            player: player,
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
        }
    }

    pub fn on_update(&mut self, e: &Input, upd: UpdateArgs) {
        self.scene.event(e);

        if self.up_d {
            self.player.mov(&mut self.scene, 0.0, -1500.0 * upd.dt);
        }
        if self.down_d {
            self.player.mov(&mut self.scene, 0.0, 1500.0 * upd.dt);
        }
        if self.left_d {
            self.player.mov(&mut self.scene, -1500.0 * upd.dt, 0.0);
        }
        if self.right_d {
            self.player.mov(&mut self.scene, 1500.0 * upd.dt, 0.0);
        }
    }

    pub fn on_draw(&mut self, e: &Input, _: RenderArgs, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            self.scene.draw(c.transform, g);
        });
    }

    pub fn on_input(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
