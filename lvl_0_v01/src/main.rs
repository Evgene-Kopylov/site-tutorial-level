#![allow(unused)] // FIXME

use macroquad::prelude::{clear_background, next_frame, Conf};
mod settings;
use settings::{window_conf, GROUND_COLOR};
mod assets;
mod scene;
use crate::scene::Scene;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        clear_background(GROUND_COLOR);
        next_frame().await
    }
}
