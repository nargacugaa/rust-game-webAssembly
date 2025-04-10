use macroquad::{color::Color, math::{Circle, Rect}};

pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Shape {
    pub fn rect_collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    pub fn circle_collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.size / 2.0,
        }
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
