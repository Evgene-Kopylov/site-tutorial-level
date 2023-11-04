//! юнит под контролем игрока

use crate::command::Command;
use crate::settings::*;
use macroquad::prelude::*;


pub struct MainUnit {
    pub texture: Texture2D,
    pub size: Vec2,
    pub scale: f32,
    pub radius: f32,
    pub rotation: f32,
    pub position: Vec2,
    pub speed: f32,
    pub shoot_timer: f32,
    shoot_delay: f32,
    pub shoot_range: f32,
    pub auto_aim: bool,
    bullet_load: u8,
}

impl MainUnit {
    pub fn new(texture: Texture2D, position: Vec2) -> Self {
        Self {
            texture,
            position,
            size: Vec2::new(texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            speed: MAIN_UNIT_SPEED,
            shoot_timer: 0.,
            shoot_delay: MAIN_UNIT_SHOOT_DELAY,
            shoot_range: MAIN_UNIT_SHOOT_RANGE,
            auto_aim: false,
            bullet_load: 0,
        }
    }

    // Возвращает сигнал о попадании в цель
    pub fn update(&mut self, dt: f32, target_point: Vec2, command: &mut Command) {
        self.shoot_timer += dt;

        self.position.x += command.wasd.x * dt * self.speed;
        self.position.y += command.wasd.y * dt * self.speed;

        if command.wasd.x != 0. || command.wasd.y != 0. || is_mouse_button_down(MouseButton::Left) {
            self.auto_aim = false;
        }

        // поворот в сторону курсора
        self.rotation %= f32::to_radians(360.);
        let mut dx = self.position.x - target_point.x;
        if dx == 0f32 {
            dx += 1f32;
        };

        let mut dy = self.position.y - target_point.y;
        if dy == 0f32 {
            dy += 1f32;
        };

        if self.auto_aim {
            self.rotation = command.rotation;
        } else if !self.auto_aim {
            if dx >= 0f32 {
                self.rotation = (dy / dx).atan() - f32::to_radians(90.);
            } else {
                self.rotation = (dy / dx).atan() - f32::to_radians(270.);
            }
        }

        // Управление огнем
        if self.shoot_timer >= self.shoot_delay {
            if is_mouse_button_down(MouseButton::Left) {
                // ЛКМ
                command.shoot = true;
                self.bullet_load = 0;
            } else if self.bullet_load > 0 {
                // очередь
                command.shoot = true;
                self.bullet_load -= 1;
            }
        } else {
            command.shoot = false;
        }

        if command.shoot {
            self.shoot_timer = 0.;
        }
    }

    pub fn draw(&self) {
        // тень
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.5 + 3.,
            self.position.y - self.size.y * 0.5 + 4.,
            DARKGRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
        // сам юнит
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.5,
            self.position.y - self.size.y * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}
