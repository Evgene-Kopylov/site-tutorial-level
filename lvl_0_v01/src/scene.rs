use crate::assets::Assets;
use macroquad::prelude::{get_frame_time, mouse_position, Vec2};

pub struct Scene {
    mouse_position: Vec2,
    dt: f32,
    assets: Assets,
}

impl Scene {
    pub async fn new() -> Self {
        // начальные базовые параметры
        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        let assets = Assets::new().await.unwrap();

        Self {
            mouse_position,
            dt,
            assets,
        }
    }
}
