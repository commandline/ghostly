extern crate ghostly;
extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Ghostly Adventure", [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let mut game = ghostly::Game::new(&mut window);
    while let Some(e) = window.next() {
        match e {
            Input::Update(upd) => game.on_update(&e, upd, &window),
            Input::Render(ren) => game.on_draw(&e, ren, &mut window),
            _ => game.on_input(e),
        }
    }
}
