use crate::vector::*;
use crate::ray::*;

#[derive(Copy,Clone,Debug)]
pub struct Camera {
    pub position: Vector,
    pub forward: Vector,
    pub right: Vector,
    pub up: Vector,
}

impl Camera {
    pub fn shoot_ray(self, x: f32, y: f32) -> Ray {
	Ray {
	    origin: self.position,
	    direction: (self.forward + self.right * x + self.up * y).normalised(),
	}
    }
}
