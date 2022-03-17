use glam::{Mat2, Vec2, Vec3, Vec3Swizzles};
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

#[derive(Debug, Copy, Clone)]
pub struct Horseshoe {
    pub angle: f32,
    pub radius: f32,
    pub length: f32,
    pub width: f32
}

impl Sdf for Horseshoe {
    fn density(&self, mut pos: Vec2) -> f32 {
        let (sin, cos) = self.angle.sin_cos();
        pos.x = pos.x.abs();
        let l = pos.length();
        pos = Mat2::from_cols_array(&[-cos, sin, sin, cos]) * pos;

        if pos.x <= 0.0  {
            if pos.y <= 0.0 {
                pos.x = l * (-cos).signum()
            }
            pos.y = l;
        }
        pos.x = pos.x - self.length;
        pos.y = (pos.y-self.radius).abs() - self.width;

        return pos.max(Vec2::ZERO).length() + pos.max_element().min(0.0);
    }
}