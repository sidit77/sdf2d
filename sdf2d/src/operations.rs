use glam::Vec2;
use crate::Sdf;

#[derive(Debug, Copy, Clone)]
pub struct Invert<T: Sdf>(pub T);

impl<T: Sdf> Sdf for Invert<T> {
    fn density(&self, pos: Vec2) -> f32 {
        -self.0.density(pos)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Translation<T: Sdf>(pub T, pub Vec2);

impl<T: Sdf> Sdf for Translation<T> {
    fn density(&self, pos: Vec2) -> f32 {
        self.0.density(pos - self.1)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Union<L: Sdf, R: Sdf>(pub L, pub R);

impl<L: Sdf, R: Sdf> Sdf for Union<L, R> {
    fn density(&self, pos: Vec2) -> f32 {
        f32::min(self.0.density(pos), self.1.density(pos))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Subtraction<L: Sdf, R: Sdf>(pub L, pub R);

impl<L: Sdf, R: Sdf> Sdf for Subtraction<L, R> {
    fn density(&self, pos: Vec2) -> f32 {
        -f32::min(-self.0.density(pos), self.1.density(pos))
    }
}