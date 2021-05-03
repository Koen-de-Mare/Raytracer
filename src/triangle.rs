use crate::vector::*;
use crate::material::*;
use crate::ray::*;
use crate::surface_element::*;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub base: Vector,
    pub v1: Vector,
    pub v2: Vector,
    pub material: Material,
}

impl Triangle {
    pub fn normal(self) -> Vector {
	cross(self.v1, self.v2).normalised()
    }

    pub fn intersect(self, ray: Ray) -> Option<(f32, SurfaceElement)> {
	assert!(ray.direction.is_normal());
	
	let normal = self.normal();
	assert!(normal.is_normal());

	if dot(normal, ray.direction) > 0f32 { return None; } // face culling
	
	let relative_origin = ray.origin - self.base;
	
	let depth = -dot(relative_origin, normal) / dot (ray.direction, normal);
	if depth < 0f32 { return None; }
	let relative_intersection_point = relative_origin + ray.direction * depth;

	let a1 = dot(relative_intersection_point, self.v1);
	let a2 = dot(relative_intersection_point, self.v2);

	let b11 = dot(self.v1, self.v1);
	let b12 = dot(self.v1, self.v2);
	let b22 = dot(self.v2, self.v2);

	let det = b11 * b22 - b12 * b12;
	
	let c1 = (a1 * b22 - a2 * b12) / det;
	let c2 = (a2 * b11 - a1 * b12) / det;

	if c1 < -0.0000001f32 || c2 < -0.0000001f32 || c1 + c2 > 1.0000001f32 {
	    return None;
	}

	// for when interpolating between the vertices, corresponds to the base
	// let c0 = 1f32 - c1 - c2; 

	let hit_position_1 = ray.origin + ray.direction * depth;
	let hit_position_2 = self.base + self.v1 * c1 + self.v2 * c2;

	//assert!((hit_position_1 - hit_position_2).norm2() < 0.0001f32);

	if (hit_position_1 - hit_position_2).norm2() > 0.0001f32 {
	    println!("INCONSISTENT TRIANGLE INTERSECTION");
	    println!("badness: {}", (hit_position_1 - hit_position_2).norm2());
	    println!("{:#?}", self);
	    println!("{:#?}", ray);
	    println!("{}", depth);
	    println!("{:#?}", hit_position_1);
	    println!("{:#?}", hit_position_2);
	}
	
	Some((
	    depth,
	    SurfaceElement {
		position: hit_position_1,
		normal:   normal,
		material: self.material,
	    }))
    }
}
