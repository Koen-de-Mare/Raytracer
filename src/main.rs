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

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
	Color {
	    r: self.r + other.r,
	    g: self.g + other.g,
	    b: self.b + other.b,
	}
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
	Color {
	    r: self.r * scalar,
	    g: self.g * scalar,
	    b: self.b * scalar,
	}
    }	
}

impl Color {
    fn quantise(self) -> image::Rgb<u8> {
	let r: u8 = (self.r * 255f32).min(255f32).max(0f32) as u8;
	let g: u8 = (self.g * 255f32).min(255f32).max(0f32) as u8;
	let b: u8 = (self.b * 255f32).min(255f32).max(0f32) as u8;
	image::Rgb([r, g, b])
    }
}

const BLACK: Color = Color {
    r: 0f32,
    g: 0f32,
    b: 0f32,
};

#[derive(Copy, Clone, Debug)]
struct Triangle {
    base: Vector,
    v1: Vector,
    v2: Vector,
    c0: Color,
    c1: Color,
    c2: Color
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

fn intersect(ray: Ray, triangle: Triangle) -> Option<Color> {
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
	let c0 = 1f32 - c1 - c2;

	let color = triangle.c0 * c0 + triangle.c1 * c1 + triangle.c2 * c2;
	
	Some(color)
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

struct Rendering {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Rendering {
    fn rendering(width: usize, height: usize) -> Rendering {
	let mut rendering = Rendering {
	    width: width,
	    height: height,
	    pixels: Vec::new(),
	};

	rendering.pixels.resize(width * height, BLACK);

	rendering
    }

    fn get_mut_pixel(&mut self, px: usize, py: usize) -> &mut Color {
	assert!(self.pixels.len() == self.width * self.height);

	assert!(px < self.width);
	assert!(py < self.height);
	
	let n = px + py * self.width;

	self.pixels.get_mut(n).unwrap()
    }
    
    fn save(self) {
	assert!(self.pixels.len() == self.width * self.height);

	let mut quantised_pixels: Vec<u8> = Vec::new();
	quantised_pixels.resize(self.width * self.height * 3, 0u8);

	for py in 0 .. self.height {
	    for px in 0 .. self.width {
		let quantised_color = self.pixels[px + py * self.width].quantise();
		quantised_pixels[(px + py * self.width) * 3]     = quantised_color[0];
		quantised_pixels[(px + py * self.width) * 3 + 1] = quantised_color[1];
		quantised_pixels[(px + py * self.width) * 3 + 2] = quantised_color[2];
		
	    }
	}
	
	let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_vec(self.width as u32, self.height as u32, quantised_pixels).unwrap();

	img.save("test.png").unwrap();
    }
}

fn main() {
    println!("rendering...");

    let camera = Camera {
	position: Vector{x: 0f32, y: 0f32, z: 0f32},
	forward:  Vector{x: 0f32, y: 1f32, z: 0f32},
	right:    Vector{x: 1f32, y: 0f32, z: 0f32},
	up:       Vector{x: 0f32, y: 0f32, z: 1f32},
    };

    let t = Triangle {
	base: Vector{x: 0f32, y: 1f32, z: 0f32},
	v1:   Vector{x: 0.5f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 0.5f32},
	c0: Color{r: 1f32, g: 0f32, b: 0f32},
	c1: Color{r: 0f32, g: 1f32, b: 0f32},
	c2: Color{r: 0f32, g: 0f32, b: 1f32},
    };


    let width = 160;
    let height = 90;
    
    let mut rendering = Rendering::rendering(width, height);

    for py in 0 .. height {
	for px in 0 .. width {
	    let pixel = rendering.get_mut_pixel(px, py);

	    let x = 2f32 * ((px as f32) - (width  as f32 * 0.5f32)) / (height as f32);	    // using height to keep aspect ratio
	    let y = 2f32 * ((py as f32) - (height as f32 * 0.5f32)) / (height as f32);

	    let ray = camera.shoot_ray(x, y);

	    match intersect(ray, t) {
		Some(color) => {
		    *pixel = color;
		},
		None => {},
	    }
	}
    }

    rendering.save();
}
