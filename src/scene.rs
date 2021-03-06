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
    AwareImportanceSampling1, // exploits knowledge of the position of the light source
    AwareImportanceSampling2, // exploits knowledge of the position of the light source
}

//const SAMPLING_METHOD: SamplingMethod = SamplingMethod::Uniform;
//const SAMPLING_METHOD: SamplingMethod = SamplingMethod::NaiveImportanceSampling;
const SAMPLING_METHOD: SamplingMethod = SamplingMethod::AwareImportanceSampling1;

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
	assert!(surface_element.normal.is_normal());
	assert!(_direction_out.is_normal());
	assert!(recurse >= 0);
	
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
		let p: f32 = rand::random::<f32>();
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
	    SamplingMethod::AwareImportanceSampling1 => {
		let direction_sphere = (self.sphere.position - surface_element.position).normalised();
		let theta_sphere_cos = dot(surface_element.normal, direction_sphere);
		let theta_sphere = theta_sphere_cos.acos();
		
		let distance_sphere = (self.sphere.position - surface_element.position).norm();
		
		// angle between the lines from the surface element to the center of the sphere and to the edge of the sphere
		let disc_angle = (self.sphere.radius / distance_sphere).atan();
		let disc_area = 2f32 * PI * (1f32 - disc_angle.cos());
		
		let theta_max = theta_sphere + disc_angle;
		
		let brightness_ambient = 0.001f32; // TEMP <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
		let brightness_disc = (self.sphere.color.r + self.sphere.color.g + self.sphere.color.b) / 3f32;
		
		let alpha = if theta_max > 0.5f32 * PI {
		    //part of the sphere is below the horizon
		    0f32
		} else {
		    let beta = (brightness_disc / brightness_ambient - 1f32) * disc_area / PI;
		    assert!(beta > 0f32);
		    beta * theta_sphere_cos / (1f32 + beta * theta_sphere_cos)
		}.max(0f32).min(0.75f32); // artificial cap at 0.75 is hacky <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

		let ray_direction = if rand::random::<f32>() < alpha {
		    // sample towards light source
		    let cos_theta = 1f32 - rand::random::<f32>() * (1f32 - disc_angle.cos());
		    let theta = cos_theta.acos();
		    let phi = 2f32 * PI * rand::random::<f32>();
		    let (v1, v2) = direction_sphere.make_orthogonal_frame();

		    direction_sphere * cos_theta + (v1 * phi.cos() + v2 * phi.sin()) * theta.sin()
		} else {
		    // sample at random, cosine weighed
		    let theta: f32 = rand::random::<f32>().acos();
		    let omega: f32 = 2f32 * PI * rand::random::<f32>();

		    let (v1, v2) = surface_element.normal.make_orthogonal_frame();
		    surface_element.normal * theta.cos() + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin()
		};
		assert!(ray_direction.is_normal());
		
		let ray = Ray {
		    origin: surface_element.position,
		    direction: ray_direction,
		};

		// notice that this code is the same as will be used by ray tracing,
		// so most importantly will never give a false negative
		let towards_disc = self.sphere.intersect(ray).is_some();

		let cos_theta_in = dot(ray_direction, surface_element.normal);
		
		let denominator = (1f32 - alpha) * cos_theta_in + if towards_disc {
		     alpha * PI / disc_area
		} else {
		    0f32
		};

		let flux_in = self.trace_ray(ray, recurse - 1);

		let flux_out = flux_in * surface_element.material.diffuse_color * (cos_theta_in / denominator);

		flux_out
	    },
	    SamplingMethod::AwareImportanceSampling2 => {
		// calculate the disc related to what part of the light source is above the horizon
		// in case the light souce is entirely below the horizon, defaults to a zero-width cone allong the normal
		let (direction_disc, disc_angle) = {
		    let direction_sphere = (self.sphere.position - surface_element.position).normalised();
		    let cos_theta_sphere = dot(surface_element.normal, direction_sphere);
		    let theta_sphere = cos_theta_sphere.acos();
		    
		    let distance_sphere = (self.sphere.position - surface_element.position).norm();

		    // angle between the lines from the surface element to the center of the sphere and to the edge of the sphere
		    let apparent_angle = (self.sphere.radius / distance_sphere).atan();

		    let theta_min = theta_sphere - apparent_angle;

		    if theta_min > PI * 0.5f32 {
			// sphere is not visible at all
			(surface_element.normal, 0f32)
		    } else {
			let theta_max = (theta_sphere + apparent_angle).min(PI * 0.5f32);
			let theta_disc = (theta_max + theta_min) * 0.5f32;
			let disc_angle   = (theta_max - theta_min) * 0.5f32;

			let direction_sphere_in_plane = (direction_sphere - surface_element.normal * cos_theta_sphere).normalised();

			let direction_disc = surface_element.normal * theta_disc.cos() + direction_sphere_in_plane * theta_disc.sin();

			(direction_disc, disc_angle)
		    }
		};
		
		let brightness_disc = (self.sphere.color.r + self.sphere.color.g + self.sphere.color.b) / 3f32;
		let area_disc = 2f32 * PI * (1f32 - disc_angle.cos());
		
		let brightness_ambient = 0.01f32; //TEMP <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
		
		//make sure alpha behaves appropriately when area_disc goes to zero
		let mut alpha =
		    (brightness_disc - brightness_ambient) / (
			brightness_disc - brightness_ambient * (1f32 - PI / (area_disc * dot(direction_disc, surface_element.normal)))
		    );
		alpha = alpha.max(0f32);
		alpha = alpha.min(0.25f32);     // TEMP taking .min(0.2f32) is a hack <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

		alpha = 0.25f32; // EXTREME HACK <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
		
		assert!(alpha <= 1f32);
		if disc_angle == 0f32 {
		    assert!(alpha == 0f32);
		}

		let direction_in = if rand::random::<f32>() < alpha {
		    // sample on the disc pointing towards the light source
		    assert!(disc_angle > 0f32);

		    let p1 = rand::random::<f32>();
		    let cos_theta = 1f32 - p1 * (1f32 - disc_angle.cos());
		    let theta = cos_theta.acos();

		    let p2 = rand::random::<f32>();
		    let omega = 2f32 * PI * p2;

		    let (v1, v2) = direction_disc.make_orthogonal_frame();
		    direction_disc * cos_theta + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin()
		} else {
		    // sample cosine-weighed
		    let p: f32 = rand::random::<f32>();
		    let theta: f32 = (1f32 - 2f32 * p).acos() / 2f32;

		    let omega: f32 = 2f32 * PI * rand::random::<f32>();

		    let (v1, v2) = surface_element.normal.make_orthogonal_frame();
		    let direction_in = surface_element.normal * theta.cos() + (v1 * omega.cos() + v2 * omega.sin()) * theta.sin();

		    direction_in
		};
		assert!(direction_in.is_normal());
		
		let towards_disc = dot(direction_in, direction_disc) > disc_angle.cos();

		let cos_theta_in = dot(direction_in, surface_element.normal);
		let denominator = if towards_disc {
		    1f32 + alpha * (PI / (area_disc * cos_theta_in) - 1f32)
		} else {
		    1f32 - alpha
		};
		
		let ray = Ray {
		    origin: surface_element.position,
		    direction: direction_in,
		};

		let flux_in = self.trace_ray(ray, recurse - 1);
		let flux_out = flux_in * surface_element.material.diffuse_color * (cos_theta_in / denominator);

		flux_out
	    },
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
