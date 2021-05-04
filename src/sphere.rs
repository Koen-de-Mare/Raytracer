use crate::vector::*;
use crate::material::*;
use crate::ray::*;
use crate::color::*;


#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub position: Vector,
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    pub fn intersect(self, ray: Ray) -> Option<f32> {
	assert!(ray.direction.is_normal());

	let relative_origin = ray.origin - self.position;
	
	// a = 1, so not computed
	let b = 2f32 * dot(relative_origin, ray.direction);
	let c = relative_origin.norm2() - self.radius * self.radius;

	let discriminant = b * b - 4f32 * c;

	if discriminant < 0f32 {
	    return None;
	}

	let distance = 0.5f32 * (-b - discriminant.sqrt());

	if distance < 0f32 {
	    return None;
	}

	//let intersection_point = ray.origin + ray.direction * distance;
	//let normal = (intersection_point - self.position).normalised();
	
	Some(distance)
    }
}
