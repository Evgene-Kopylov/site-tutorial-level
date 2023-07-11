// #![allow(unused)] // FIXME

use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod assets;
mod enemy_unit;
mod main_unit;
mod command;
mod scene;
mod target_unit;
mod utils;

use crate::scene::Scene;
use main_unit::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        scene.update();
        clear_background(GROUND_COLOR);
        scene.draw();
        next_frame().await
    }
}
