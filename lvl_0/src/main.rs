// #![allow(unused)] // FIXME

use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod main_unit;
mod target_unit;
mod enemy_unit;
mod scene;
mod utils;
mod assets;
mod order;

use main_unit::*;
use crate::scene::Scene;


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
