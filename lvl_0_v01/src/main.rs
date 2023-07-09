#![allow(unused)] // FIXME

use macroquad::prelude::{
    Conf,
    next_frame,
    clear_background,
};
mod settings;
use settings::{
    GROUND_COLOR,
    window_conf,
};


#[macroquad::main(window_conf)]
async fn main() {

    loop {
        clear_background(GROUND_COLOR);
        next_frame().await
    }
}