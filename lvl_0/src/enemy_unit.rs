use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::color::{BLACK, GREEN, WHITE};
use macroquad::prelude::{Color, draw_texture_ex, DrawTextureParams, Texture2D, BROWN};
use crate::{TARGET_UNIT_IMPACT_SOUND_VOLUME, Vec2};
use crate::settings::*; // FIXME

#[derive(Clone)]
pub struct EnemyUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub shift: Vec2,
    impact_sound: Sound,
    pub(crate) hit_points: f32,
    alive: bool,
}

impl EnemyUnit {
    pub fn new(texture: Texture2D, shadow_texture: Texture2D, 
        impact_sound: Sound, spawn_position: Vec2) -> Self {
        let color = BLACK;
        // color.a = 0.45;

        Self {
            texture,
            shadow_texture,
            color,
            position: spawn_position,
            rotation: f32::to_radians(180.0),
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
            color = BROWN;  // FIXME
        } else {
            color = GREEN;
        }

        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + self.shift.x,
            self.position.y - self.texture.height() * 0.5 - self.shift.y,
            color,
            DrawTextureParams {
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            }
        );
    }

    pub fn draw_shadow(&self) {
        // тень
        let height = 1.6;
        let mut color = self.color;
        color.a = 0.2;
        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + 3. * height,
            self.position.y - self.texture.height() * 0.5 + 4. * height,
            color,
            DrawTextureParams {
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            }
        );
    }

    pub fn update(&mut self, dt: f32, target: Vec2) {
        self.rotation = self.rotation % f32::to_radians(360.);
        let mut dx = self.position.x - target.x;
        if dx == 0f32 {
            dx += 1f32;
        };

        let mut dy = self.position.y - target.y;
        if dy == 0f32 {
            dy += 1f32;
        };

        // угол к целиwww
        let a;
        if dx >= 0f32 {
            a = (dy / dx).atan();
        } else {
            a = (dy / dx).atan() - f32::to_radians(180.);
        }

        // изменение угла поворота
        let mut da = self.rotation - a;
        if da <= f32::to_radians(-180.) {
            da += f32::to_radians(360.)
        }
        if da > f32::to_radians(180.) {
            da -= f32::to_radians(360.)
        }

        // сохранение направления движения
        if da.abs() > f32::to_radians(9.) {
            if da > 0. {
                self.rotation -= dt * ENEMY_UNIT_ROTATION_SPEED
            } else {
                self.rotation += dt * ENEMY_UNIT_ROTATION_SPEED
            }
        }

        self.position.x += -1. * dt * ENEMY_UNIT_SPEED * self.rotation.cos();
        self.position.y += -1. * dt * ENEMY_UNIT_SPEED * self.rotation.sin();

    }

}