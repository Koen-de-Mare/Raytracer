// om te checken; C-c C-c (twee keer caps-lock + c)
// voor andere rust commands; M-x met rust-check, rust-compile, rust-test, rust-run

extern crate image;

use std::ops::*;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
	Vector {
	    x: self.x + other.x,
	    y: self.y + other.y,
	    z: self.z + other.z,
	}
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
	Vector {
	    x: self.x - other.x,
	    y: self.y - other.y,
	    z: self.z - other.z,
	}
    }
}

impl Neg for Vector {
    type Output = Vector;
    
    fn neg(self) -> Vector {
	Vector {
	    x: -self.x,
	    y: -self.y,
	    z: -self.z,
	}
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
	Vector {
	    x: self.x * scalar,
	    y: self.y * scalar,
	    z: self.z * scalar,
	}
    }	
}

impl Vector {
    fn norm(self) -> f32 {
	dot(self, self).sqrt()
    }
    
    fn normalised(self) -> Vector {
	self * (1f32 / self.norm())
    }
}

fn dot(v1: Vector, v2: Vector) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn cross(v1: Vector, v2: Vector) -> Vector {
    Vector {
	x: v1.y * v2.z - v1.z * v2.y,
	y: v1.z * v2.x - v1.x * v2.z,
	z: v1.x * v2.y - v1.y * v2.x,
    }
}

#[derive(Copy, Clone, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

// TODO Add, Mul



#[derive(Copy, Clone, Debug)]
struct Triangle {
    base: Vector,
    v1: Vector,
    v2: Vector,
}

impl Triangle {
    fn normal(self) -> Vector {
	cross(self.v1, self.v2).normalised()
    }
}

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: Vector,
    direction: Vector
}

fn intersect(ray: Ray, triangle: Triangle) -> Option<f32> {
    let normal = triangle.normal();

    let relative_origin = ray.origin - triangle.base;
    
    let lambda = dot(relative_origin, ray.direction) / dot(ray.direction, ray.direction);
    let relative_intersection_point = relative_origin + ray.direction * lambda;

    let a1 = dot(relative_intersection_point, triangle.v1);
    let a2 = dot(relative_intersection_point, triangle.v2);

    let b11 = dot(triangle.v1, triangle.v1);
    let b12 = dot(triangle.v1, triangle.v2);
    let b22 = dot(triangle.v2, triangle.v2);

    let det = b11 * b22 - b12 * b12;
    
    let c1 = (a1 * b22 - a2 * b12) / det;
    let c2 = (a2 * b11 - a1 * b22) / det;

    if c1 > 0f32 && c2 > 0f32 && c1 + c2 < 1f32 {
	Some(lambda)
    } else {
	None
    }
}

#[derive(Copy,Clone,Debug)]
struct Camera {
    position: Vector,
    forward: Vector,
    right: Vector,
    up: Vector,
}

impl Camera {
    fn shoot_ray(self, x: f32, y: f32) -> Ray {
	Ray {
	    origin: self.position,
	    direction: (self.forward + self.right * x + self.up * y).normalised(),
	}
    }
}

fn main() {
    println!("Hello, world!");

    let camera = Camera {
	position: Vector{x: 0f32, y: 0f32, z: 0f32},
	forward:  Vector{x: 0f32, y: 1f32, z: 0f32},
	right:    Vector{x: 1f32, y: 0f32, z: 0f32},
	up:       Vector{x: 0f32, y: 0f32, z: 1f32},
    };

    let t = Triangle {
	base: Vector{x: 0f32, y: 1f32, z: 0f32},
	v1:   Vector{x: 1f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 1f32},
    };


    
    let img: RgbImage = ImageBuffer::new(512, 512);

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
	let x2 = x as f32 * 2f32 / 512f32 - 1f32;
	let y2 = y as f32 * 2f32 / 512f32 - 1f32;
	let ray = camera.shoot_ray(x2, y2);

	match intersect(ray, t) {
	    Some(depth) => image::Rgb([0u8, 200u8, 0u8]),
	    None => image::Rgb([0u8, 0u8, 0u8]),
	}
	/*
	if x + y + y < 300 {
            //image::Luma([0u8])
	    image::Rgb([0u8, 0u8, 255u8])
	} else {
            //image::Luma([255u8])
	    image::Rgb([0u8, 255u8, 0u8])
	}*/
    });

    img.save("test.png").unwrap();
}
