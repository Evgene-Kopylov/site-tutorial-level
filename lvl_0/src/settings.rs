use macroquad::prelude::{Color, Conf};


pub const GROUND_COLOR: Color = Color::new(0.77, 0.8, 0.8, 1.00);
pub const UNIT_COLOR: Color = Color::new(0.94, 0.94, 0.94, 1.);
pub const PROJECTILE_COLOR: Color = Color::new(1.00, 0.96, 0.84, 1.00);
pub const TRANSPARANT: Color = Color::new(0., 0., 0., 0.);

pub const MAIN_UNIT_SPEED: f32 = 300.;
pub const MAIN_UNIT_SHOOT_DELAY: f32 = 0.1;
pub const MAIN_UNIT_SHOOT_RANGE: f32 = 3000.;
pub const MAIN_UNIT_SHOOT_SOUND_VOLUME: f32 = 0.16;
pub const TARGET_UNIT_IMPACT_SOUND_VOLUME: f32 = 0.345;

pub const ENEMY_UNIT_ROTATION_SPEED: f32 = 4.0;
pub const ENEMY_UNIT_SPEED: f32 = 130.0;
pub const ENEMY_UNIT_IMPACT_SOUND_VOLUME: f32 = 0.08;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("lvl_0"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}
