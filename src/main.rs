// om te checken; C-c C-c (twee keer caps-lock + c)
// voor andere rust commands; M-x met rust-check, rust-compile, rust-test, rust-run

extern crate image;
extern crate rand;

use std::ops::*;

use image::ImageBuffer;

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
    fn norm2(self) -> f32 {
	dot(self, self)
    }

    fn norm(self) -> f32 {
	dot(self, self).sqrt()
    }
    
    fn normalised(self) -> Vector {
	self * (1f32 / self.norm())
    }

    fn is_normal(self) -> bool {
	let norm2 = self.norm2();

	norm2 < 1.0001f32 && norm2 > 0.9999f32
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

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
	Color {
	    r: self.r * other.r,
	    g: self.g * other.g,
	    b: self.b * other.b,
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
    material: Material,
}

impl Triangle {
    fn normal(self) -> Vector {
	cross(self.v1, self.v2).normalised()
    }

    fn intersect(self, ray: Ray) -> Option<(f32, SurfaceElement)> {
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

	assert!((hit_position_1 - hit_position_2).norm2() < 0.001f32);
	
	Some((
	    depth,
	    SurfaceElement {
		position: hit_position_1,
		normal:   normal,
		material: self.material,
	    }))
    }
}

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: Vector,
    direction: Vector // must be a unit vector
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

#[derive(Clone, Debug)]
struct Scene {
    triangles: Vec<Triangle>
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

	let (v1, v2) = surface_element.make_frame();
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
    
    fn trace_ray(&self, ray: Ray, recurse: i32) -> Color {
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

#[derive(Copy, Clone, Debug)]
struct Material {
    diffuse_color: Color,
    emmitance_color: Color,
}

#[derive(Copy, Clone, Debug)]
struct SurfaceElement {
    position: Vector,
    normal: Vector, // must be a unit vector
    material: Material,
}

impl SurfaceElement {
    // makes two mutually orthogonal vectors, both orthogonal to the normal vector
    fn make_frame(self) -> (Vector, Vector) {
	assert!(self.normal.is_normal());
	
	let vec_start = if self.normal.x.abs() < 0.8f32 {
	    Vector {
		x: 1f32,
		y: 0f32,
		z: 0f32,
	    }
	} else {
	    Vector {
		x: 0f32,
		y: 1f32,
		z: 0f32,
	    }
	};

	let v1 = cross(self.normal, vec_start).normalised();
	let v2 = cross(self.normal, v1).normalised();

	assert!(dot(self.normal, v1).abs() < 0.0001f32);
	assert!(dot(self.normal, v2).abs() < 0.0001f32);
	assert!(dot(v1,          v2).abs() < 0.0001f32);
	
	(v1, v2)
    }
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    position: Vector,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn intersect(self, ray: Ray) -> Option<(f32, SurfaceElement)> {
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

	let intersection_point = ray.origin + ray.direction * distance;

	let surface_element = SurfaceElement {
	    position: intersection_point,
	    normal: (intersection_point - self.position).normalised(),
	    material: self.material,
	};
	
	Some((distance, surface_element))
    }
}


fn main() {
    println!("rendering...");

    let camera = Camera {
	position: Vector{x: 0.001f32, y: -0.801f32, z: 1.0001f32},
	forward:  Vector{x: 0f32, y: 1f32, z: 0f32},
	right:    Vector{x: 1f32, y: 0f32, z: 0f32},
	up:       Vector{x: 0f32, y: 0f32, z: 1f32},
    };

    let mut scene = Scene {
	triangles: Vec::new(),
    };

    //floor
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 0f32},
	v1:   Vector{x: -2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: -2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.6f32,
		g: 0.6f32,
		b: 0.6f32,
	    },
	    emmitance_color: BLACK,
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.6f32,
		g: 0.6f32,
		b: 0.6f32,
	    },
	    emmitance_color: BLACK,
	},
    });

    //ceiling
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: -2f32, z: 0f32},
	v2:   Vector{x: -2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	    emmitance_color: Color {
		r: 1f32,
		g: 1f32,
		b: 1f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 2f32, z: 0f32},
	v2:   Vector{x: 2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	    emmitance_color: Color {
		r: 1f32,
		g: 1f32,
		b: 1f32,
	    },
	},
    });


    //walls
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 0f32, z: -2f32},
	v2:   Vector{x: 0f32, y: -2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.2f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 0f32, z: 2f32},
	v2:   Vector{x: 0f32, y: 2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.2f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });

    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: -2f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: -2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.2f32,
		g: 0.9f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 2f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.2f32,
		g: 0.9f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });


    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 0f32, z: 2f32},
	v2:   Vector{x: -2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 0f32, z: -2f32},
	v2:   Vector{x: 2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.2f32,
	    },
	    emmitance_color: BLACK,
	},
    });

    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: -2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	    emmitance_color: BLACK,
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 2f32},
	v1:   Vector{x: 2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: -2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	    emmitance_color: BLACK,
	},
    });

    
    
    let width = 160 * 4;
    let height = 90 * 4;
    
    let mut rendering = Rendering::rendering(width, height);

    for py in 0 .. height {
	println!("{} out of {}...", py, height);
	for px in 0 .. width {
	    let pixel = rendering.get_mut_pixel(px, py);

	    let x = 2f32 * ((px as f32) - (width  as f32 * 0.5f32)) / (height as f32);	    // using height to keep aspect ratio
	    let y = -2f32 * ((py as f32) - (height as f32 * 0.5f32)) / (height as f32);

	    let ray = camera.shoot_ray(x, y);

	    let num_samples = 100;
	    let mut accumulator = BLACK;
	    for _ in 0 .. num_samples {
		accumulator = accumulator + scene.trace_ray(ray, 5);
	    }
	    
	    *pixel = accumulator * (1f32 / num_samples as f32);
	}
    }

    println!("saving...");
    
    rendering.save();
}
