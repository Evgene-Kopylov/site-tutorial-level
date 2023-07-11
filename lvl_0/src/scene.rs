use macroquad::audio::{self, PlaySoundParams};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{
    info, mouse_position, screen_height, 
    screen_width, Vec2};
use macroquad::time::get_frame_time;
use quad_url::set_program_parameter;
use crate::MainUnit;
use crate::enemy_unit::EnemyUnit;
use crate::projectile::Projectile;
use crate::assets::Assets;
use crate::order::Order;
use crate::settings::ENEMY_UNIT_IMPACT_SOUND_VOLUME;
use crate::target_unit::TargetUnit;
use crate::utils::get_parameter_value;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
    enemy_units: Vec<EnemyUnit>,
    projectiles: Vec<Projectile>,
    dt: f32,
    assets: Assets,
    order: Order,
    tick: f32,
    target_point: Vec2,
}

impl Scene {
    pub async fn new() -> Self {
        let spawn_position = Vec2::new(screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_position = Vec2::new(screen_width() * 0.5, 160.);


        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        let assets = Assets::new().await.unwrap();

        let mut enemy_unit_0 = EnemyUnit::new(
            assets.enemy_unit_gray,
            assets.target_unit_shadow_texture,
            assets.target_impact_sound,
            target_unit_position
        );
        enemy_unit_0.rotation += f32::to_radians(90.);

        let mut enemy_unit_1 = enemy_unit_0.clone();
        enemy_unit_1.position.y -= 150.;
        enemy_unit_1.position.x += 150.;
        let mut enemy_unit_2 = enemy_unit_0.clone();
        enemy_unit_2.position.y -= 150.;
        enemy_unit_2.position.x += 250.;
        // let mut enemy_unit_3 = enemy_unit_0.clone();
        // enemy_unit_3.position.x += 350.;

        let mut enemy_unit_4 = enemy_unit_0.clone();
        enemy_unit_4.position.x -= 150.;
        let mut enemy_unit_5 = enemy_unit_0.clone();
        enemy_unit_5.position.x -= 250.;
        // let mut enemy_unit_6 = enemy_unit_0.clone();
        // enemy_unit_6.position.x -= 350.;

        let mut enemy_unit_7 = enemy_unit_0.clone();
        enemy_unit_7.position.y += 150.;
        enemy_unit_7.position.x -= 150.;
        let mut enemy_unit_8 = enemy_unit_0.clone();
        enemy_unit_8.position.y += 150.;
        // enemy_unit_8.position.x -= 250.;
        let mut enemy_unit_9 = enemy_unit_0;
        enemy_unit_9.position.y += 150.;
        enemy_unit_9.position.x += 150.;

        let enemy_units = vec![
            enemy_unit_1, enemy_unit_2, 
            // enemy_unit_3,
            enemy_unit_4, enemy_unit_5, 
            // enemy_unit_6,
            enemy_unit_7, enemy_unit_8, enemy_unit_9,
            ];

        Self {
            main_unit: MainUnit::new(
                assets.main_unit_texture,
                spawn_position
            ),
            target_unit: TargetUnit::new(
                assets.target_unit_texture,
                assets.target_unit_shadow_texture,
                assets.target_impact_sound,
                target_unit_position
            ),
            enemy_units,
            projectiles: vec![],
            dt,
            assets,
            order: Order::new(),
            tick: 1000.,  // большое число, чтобы сразу срабатывало
            target_point: mouse_position,
        }
    }


    /// Поймать активность пользователя.
    fn update_order_from_user_input(&mut self) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D){
            x_move += 1f32;
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y_move += 1f32;
        }

        if self.main_unit.position.x < 1f32 {
            x_move = 1f32;
        }
        if self.main_unit.position.x > screen_width() {
            x_move = -1f32;
        }

        if self.main_unit.position.y < 1f32 {
            y_move = 1f32;
        }
        if self.main_unit.position.y > screen_height() {
            y_move = -1f32;
        }
        self.order.wasd = Vec2::new(x_move, y_move);
    }

    fn update_order_from_url_query(&mut self) {
        match get_parameter_value("command") == *"Shoot" {
            true => {
                self.order.shoot = true;
                let x = get_parameter_value("target_point_x").parse().unwrap_or(0.);
                let y = get_parameter_value("target_point_y").parse().unwrap_or(0.);
                self.target_point = Vec2::new(x, y);
                info!("{:?}", self.target_point);
                set_program_parameter("command", "");
                self.main_unit.shoot_timer = 1.;  // чтобы получить выстрел с минимальной задержкой
                self.main_unit.auto_aim = true;
            }
            false => {}
        }

        match get_parameter_value("rotation").parse::<f32>() {
            Ok(a) => {
                self.order.rotation = a.to_radians();
            }
            Err(_e) => {
                // info!("{}", _e);
            }
        }

    }

    fn set_parameters_to_url_query(&mut self) {
        let line = format!("({}, {})", self.target_unit.position.x as i32, self.target_unit.position.y as i32);
        set_program_parameter("target_pos", line.as_str());
        let line = format!("({}, {})", self.main_unit.position.x as i32, self.main_unit.position.y as i32);
        set_program_parameter("unit_pos", line.as_str());
        
        let mut line = "[".to_string();
        for i in 0..self.enemy_units.len() {
            let value = format!("({}, {}), ",
                self.enemy_units[i].position.x as i32,
                self.enemy_units[i].position.y as i32
            );
            line += &value;
        }
        line += "]";
        set_program_parameter("enemy_units", &line);
    }


    pub fn update(&mut self) {
        self.tick += self.dt;
        self.update_order_from_user_input();

        if self.tick >= 1. {
            self.tick = 0.0;
            self.set_parameters_to_url_query();
            self.update_order_from_url_query();
        }
        self.dt = get_frame_time();
        self.target_unit.shift = Vec2::new(0., 0.);

        let target_point = if self.target_point.x != 0. 
        || self.target_point.y != 0. {
            self.target_point
        } else {
            mouse_position().into()
        };

        self.main_unit.update(
            self.dt,
            target_point,
            &mut self.order,
        );
        if self.order.shoot {
            let position = Vec2::new(  // точка появления выстрела
                self.main_unit.position.x + 65. * (self.main_unit.rotation - f32::to_radians(90.)).cos(),
                self.main_unit.position.y + 65. * (self.main_unit.rotation - f32::to_radians(90.)).sin()
            );

            let projectile = Projectile::new(
                self.assets.projectile_texture,
                self.assets.main_unit_shoot_sound,
                self.main_unit.rotation,
                position,
                self.main_unit.speed * 3.,
            );
            self.projectiles.push(projectile);
        }

        // удалить дохлые юниты
        self.enemy_units.retain(|u| u.hit_points > 0.);

        // update enemy units
        for i in 0..self.enemy_units.len() {
            let units = self.enemy_units.clone();
            self.enemy_units[i].update(
                self.dt, 
                self.main_unit.position,
                units,
                i
            );
        }

        // удаление объектов
        // снаряды на отлете
        self.projectiles.retain(|p|
                ((p.start_position.x - p.position.x).powf(2f32)
                    + (p.start_position.y - p.position.y).powf(2f32)
                    < self.main_unit.shoot_range.powf(2f32)) && p.alive);
        
        // поражение главной мишени
        for i in 0..self.projectiles.len() {
            let p = &mut self.projectiles[i];

            if (p.position.x - self.target_unit.position.x).powf(2f32) +
                (p.position.y - self.target_unit.position.y).powf(2f32)
                < self.target_unit.radius.powf(2f32) {
                p.alive = false;
                self.target_unit.update(
                    true,
                    -20.,
                    p.rotation,
                );
                info!("target_unit.hit_points: {:?}", self.target_unit.hit_points);
            }

            p.update(self.dt);
        }

        // поражение enemy_units
        for i in 0..self.projectiles.len() {
            let p = &mut self.projectiles[i];
            for j in 0..self.enemy_units.len() {
                let u = &mut self.enemy_units[j];
                let dx = p.position.x - u.position.x;
                let dy = p.position.y - u.position.y;
                let dist = (dx.powf(2.) + dy.powf(2.)).sqrt();
                if dist < u.radius {
                    u.hit_points -= 20.;
                    audio::play_sound(u.impact_sound, PlaySoundParams {
                        volume: ENEMY_UNIT_IMPACT_SOUND_VOLUME, ..Default::default() });

                    let da = u.rotation - p.rotation;
                    p.alive = false;
                    u.rotation += (da.abs() / da) * f32::to_radians(20.);
                }
            }

        }

    }

    pub fn draw(&self) {
        self.target_unit.draw_shadow();
        self.main_unit.draw();
        for i in 0..self.enemy_units.len() {
            self.enemy_units[i].draw_shadow();
            self.enemy_units[i].draw();
            // self.enemy_units[i].draw_front();
        }
        for i in 0..self.projectiles.len() {
            self.projectiles[i].draw();
        }
        self.target_unit.draw();
    }

}