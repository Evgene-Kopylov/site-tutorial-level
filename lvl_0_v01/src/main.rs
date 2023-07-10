#![allow(unused)] // FIXME
mod settings;
mod assets;
mod scene;
mod units;

use macroquad::prelude::{clear_background, next_frame, Conf};
use settings::{window_conf, GROUND_COLOR};

use crate::scene::Scene;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        clear_background(GROUND_COLOR);
        next_frame().await
    }
}
