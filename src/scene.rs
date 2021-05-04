use crate::triangle::*;
use crate::surface_element::*;
use crate::ray::*;
use crate::vector::*;
use crate::color::*;
use crate::sphere::*;

use std::f32::consts::PI;

enum SamplingMethod {
    Uniform,
    NaiveImportanceSampling, // cosine weighed, not aware of the position of the light source
    //AwareImportanceSampling, // exploits knowledge of the position of the light source
}

const SAMPLING_METHOD: SamplingMethod = SamplingMethod::Uniform;
//const SAMPLING_METHOD: SamplingMethod = SamplingMethod::NaiveImportanceSampling;

#[derive(Clone, Debug)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub sphere: Sphere,
}

impl Scene {
    fn scan_triangles(&self, ray: Ray) -> Option<(f32,SurfaceElement)> {
	let mut best_hit = None;
	let mut best_depth = 99999999f32;

	for triangle in &self.triangles {
	    match triangle.intersect(ray) {
		None => {},
		Some((depth, surface_element)) => {
		    if depth < best_depth {
			best_depth = depth;
			best_hit = Some((depth,surface_element));
		    }
		},
	    }
	}

	best_hit
    }

    // finds the light leaving the surface element in the specified direction
    // convention for direction_out to be the direction INTO the surface
    // convention for direction_in to be OUT OF the surface
    // i.e. both in the direction of ray tracing, and opposite to the direction of the light
    fn light_out(&self, surface_element: SurfaceElement, _direction_out: Vector, recurse: i32) -> Color {
	if recurse == 0 {
	    return BLACK;
	}

	match SAMPLING_METHOD {
	    SamplingMethod::Uniform => {
		let p: f32 = rand::random();
		let theta: f32 = p.acos();
		let omega: f32 = 2f32 * PI * rand::random::<f32>();

		let (v1, v2) = surface_element.normal.make_orthogonal_frame();
		let direction_in = surface_element.normal * theta.cos() + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin();
		assert!(direction_in.is_normal());
		
		let ray = Ray {
		    origin: surface_element.position,
		    direction: direction_in,
		};

		let flux_in = self.trace_ray(ray, recurse - 1);

		flux_in * surface_element.material.diffuse_color * (theta.cos() * 2f32)
	    },
	    SamplingMethod::NaiveImportanceSampling => {
		let p: f32 = rand::random();
		let theta: f32 = (1f32 - 2f32 * p).acos() / 2f32;

		let omega: f32 = 2f32 * PI * rand::random::<f32>();

		let (v1, v2) = surface_element.normal.make_orthogonal_frame();
		let direction_in = surface_element.normal * theta.cos() + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin();
		assert!(direction_in.is_normal());
		
		let ray = Ray {
		    origin: surface_element.position,
		    direction: direction_in,
		};

		let flux_in = self.trace_ray(ray, recurse - 1);

		flux_in * surface_element.material.diffuse_color
	    },
	    //AwareImportanceSampling => {},
	}
    }
    
    pub fn trace_ray(&self, ray: Ray, recurse: i32) -> Color {
	assert!(recurse >= 0);
	
	let triangle_hit = self.scan_triangles(ray);
	let sphere_hit = self.sphere.intersect(ray);
	
	
	match (triangle_hit, sphere_hit) {
	    (None, None) => { BLACK },
	    (Some((_,surface_element)), None) => {
		assert!(surface_element.normal.is_normal());
		self.light_out(surface_element, ray.direction, recurse)
	    },
	    (None, Some(_)) => {
		self.sphere.color
	    },
	    (Some((triangle_depth, surface_element)), Some(sphere_depth)) => {
		if triangle_depth < sphere_depth {
		    assert!(surface_element.normal.is_normal());
		self.light_out(surface_element, ray.direction, recurse)
		} else {
		    self.sphere.color
		}
	    },
	}
    }
}
