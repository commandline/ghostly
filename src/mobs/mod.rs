pub use self::hero::Hero;
pub use self::star::Star;

mod hero;
mod star;

// TODO use window::size
pub fn wrap(win: (f64, f64), bounds: (f64, f64), pos: (f64, f64)) -> (bool, f64, f64) {
    let new_x = wrap_dim(win.0, bounds.0, pos.0);
    let new_y = wrap_dim(win.1, bounds.1, pos.1);
    (new_x != pos.0 || new_y != pos.1, new_x, new_y)
}

fn wrap_dim(win_dim: f64, sprite_dim: f64, pos_dim: f64) -> f64 {
    if pos_dim > win_dim + sprite_dim / 2.0 {
        -sprite_dim / 2.0
    } else if pos_dim < -sprite_dim / 2.0 {
        win_dim + sprite_dim / 2.0
    } else {
        pos_dim
    }
}
