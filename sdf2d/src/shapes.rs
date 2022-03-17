use glam::{Vec2, Vec3, Vec3Swizzles};
use crate::Sdf;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub radius: f32
}

impl Sdf for Circle {
    fn density(&self, pos: Vec2) -> f32 {
        pos.length() - self.radius
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hexagon {
    pub radius: f32
}

impl Sdf for Hexagon {
    fn density(&self, mut pos: Vec2) -> f32 {
        let k = Vec3::new(-0.866025404,0.5,0.577350269);
        pos = pos.abs();
        pos -= 2.0 * k.xy().dot(pos).min(0.0) * k.xy();
        pos -= Vec2::new(f32::clamp(pos.x, -k.z * self.radius, k.z * self.radius), self.radius);
        return pos.length() * f32::signum(pos.y);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32
}

impl Sdf for Rectangle {
    fn density(&self, pos: Vec2) -> f32 {
        let d = pos.abs() - Vec2::new(self.width, self.height);
        d.max(Vec2::ZERO).length() + d.max_element().min(0.0)
    }
}