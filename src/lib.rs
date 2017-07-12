extern crate ai_behavior;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

mod object;

use gfx_device_gl::Resources;
use graphics::Image;
use graphics::rectangle::square;
use piston_window::*;
use object::Object;
use sprite::*;

// TODO move this into the player object
const MV_FACT: f64 = 2500.0;

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
            self.player.mov(&mut self.scene, 0.0, -MV_FACT * upd.dt);
        }
        if self.down_d {
            self.player.mov(&mut self.scene, 0.0, MV_FACT * upd.dt);
        }
        if self.left_d {
            self.player.mov(&mut self.scene, -MV_FACT * upd.dt, 0.0);
        }
        if self.right_d {
            self.player.mov(&mut self.scene, MV_FACT * upd.dt, 0.0);
        }
    }

    pub fn on_draw(&mut self, e: &Input, _: RenderArgs, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("Fresca-Regular.ttf");
        let factory = w.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

        let image = Image::new().rect(square(0.0, 0.0, 640.0));
        let bg = Texture::from_path(&mut w.factory,
                                       assets.join("bg.png"),
                                       Flip::None,
                                       &TextureSettings::new()).unwrap();
        w.draw_2d(e, |c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([1.0, 1.0, 1.0, 1.0], g);
            for number in 0..100 {
                let x: f64 = (number % 10 * 64).into();
                let y: f64 = (number / 10 * 64).into();
                image.draw(&bg, &Default::default(), c.transform.trans(x, y).zoom(0.1), g);
            }
            if let Some(player_sprite) = self.scene.child(self.player.sprite_id) {
                let (x, y) = player_sprite.get_position();
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &format!("x: {}, y: {}", x.trunc(), y.trunc()),
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
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
