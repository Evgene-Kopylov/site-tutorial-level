use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::color::{BLACK, GREEN, WHITE};
use macroquad::prelude::{Color, draw_texture_ex, DrawTextureParams, Texture2D};
use crate::{TARGET_UNIT_IMPACT_SOUND_VOLUME, Vec2};


#[derive(Clone)]
pub struct EnemyUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: Vec2,
    pub radius: f32,
    pub shift: Vec2,
    impact_sound: Sound,
    pub(crate) hit_points: f32,
    alive: bool,
}

impl EnemyUnit {
    pub fn new(texture: Texture2D, shadow_texture: Texture2D, 
        impact_sound: Sound, spawn_position: Vec2) -> Self {
        let mut color = BLACK;
        color.a = 0.45;

        Self {
            texture,
            shadow_texture,
            color,
            position: spawn_position,
            radius: texture.width() * 0.5,
            shift: Vec2::new(0., 0.),
            impact_sound,
            hit_points: 100.,
            alive: true,
        }
    }

    pub fn draw(&self) {
        let color;
        if self.alive {
            color = self.color;
        } else {
            color = GREEN;
        }

        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + self.shift.x,
            self.position.y - self.texture.height() * 0.5 - self.shift.y,
            color,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }
}