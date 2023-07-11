use macroquad::math::Vec2;

pub struct Command {
    pub shoot: bool,
    pub wasd: Vec2, // смещение
    pub rotation: f32,
}

impl Command {
    pub fn new() -> Self {
        Self {
            shoot: false,
            wasd: Vec2::new(0., 0.),
            rotation: 0.,
        }
    }
}
