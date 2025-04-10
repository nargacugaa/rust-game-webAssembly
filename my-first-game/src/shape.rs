use macroquad::{color::Color, math::Rect};

pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Shape {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
