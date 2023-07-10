use macroquad::{Vec2, Texture2D};

pub trait Unit {
    fn pos(&self) -> Vec2;
    fn texture(&self) -> Texture2D;
    fn alive(&self) -> bool;
}
