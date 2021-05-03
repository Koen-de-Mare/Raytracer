use crate::triangle::*;
use crate::surface_element::*;
use crate::ray::*;
use crate::vector::*;
use crate::color::*;

#[derive(Clone, Debug)]
pub struct Scene {
    pub triangles: Vec<Triangle>
}

impl Scene {
    fn scan_hit(&self, ray: Ray) -> Option<SurfaceElement> {
	let mut best_hit = None;
	let mut best_depth = 99999999f32;

	for triangle in &self.triangles {
	    match triangle.intersect(ray) {
		None => {},
		Some((depth, surface_element)) => {
		    if depth < best_depth {
			best_depth = depth;
			best_hit = Some(surface_element);
		    }
		},
	    }
	}

	best_hit
    }

    // finds the light leaving the surface element in the specified direction
    // convention for direction to be the direction INTO the surface
    fn light_out(&self, surface_element: SurfaceElement, direction: Vector, recurse: i32) -> Color {
	if recurse == 0 {
	    return surface_element.material.emmitance_color;
	}

	let p: f32 = rand::random();
	let theta: f32 = (1f32 - 2f32 * p).acos() / 2f32;

	let omega: f32 = 2f32 * std::f32::consts::PI * rand::random::<f32>();

	let (v1, v2) = surface_element.normal.make_orthogonal_frame();
	let new_direction = surface_element.normal * theta.cos() + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin();

	//let new_direction = surface_element.normal;
	assert!(new_direction.is_normal());
	
	let ray = Ray {
	    origin: surface_element.position,
	    direction: new_direction,
	};

	let flux_in = self.trace_ray(ray, recurse - 1);

	flux_in * surface_element.material.diffuse_color + surface_element.material.emmitance_color
    }
    
    pub fn trace_ray(&self, ray: Ray, recurse: i32) -> Color {
	assert!(recurse >= 0);
	
	match self.scan_hit(ray) {
	    None => { BLACK },
	    Some(surface_element) => {
		assert!(surface_element.normal.is_normal());
		self.light_out(surface_element, ray.direction, recurse)
	    },
	}
    }
}
