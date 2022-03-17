mod shapes;
mod operations;

use glam::{Mat2, Vec2};
use crate::operations::{Invert, Rotation, Subtraction, Translation, Union};
use crate::shapes::{Circle, Hexagon, Rectangle};

pub trait Sdf : Sized{
    fn density(&self, pos: Vec2) -> f32;
}

pub struct Shapes;
impl Shapes {
    pub fn circle(radius: f32) -> Circle {
        Circle {
            radius
        }
    }
    pub fn hexagon(radius: f32) -> Hexagon {
        Hexagon {
            radius
        }
    }
    pub fn rectangle(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

pub trait Ops where Self: Sdf {
    fn invert(self) -> Invert<Self> {
        Invert(self)
    }
    fn translate(self, x: f32, y: f32) -> Translation<Self> {
        Translation(self, -Vec2::new(x, y))
    }
    fn rotate(self, angle: f32) -> Rotation<Self> {
        Rotation(self, Mat2::from_angle(-angle))
    }
    fn subtract<T: Sdf>(self, other: T) -> Subtraction<Self, T> {
        Subtraction(self, other)
    }
    fn union<T: Sdf>(self, other: T) -> Union<Self, T> {
        Union(self, other)
    }
}
impl<T: Sdf> Ops for T {}