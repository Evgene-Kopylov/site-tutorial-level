mod settings;

use macroquad::prelude::{
    clear_background,
    next_frame,
};

use settings::{
    GROUND_COLOR,
};


#[macroquad::main("breakout")]
async fn main() {

    loop {
        clear_background(GROUND_COLOR);
        next_frame().await
    }
}