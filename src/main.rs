// om te checken; C-c C-c (twee keer caps-lock + c)
// voor andere rust commands; M-x met rust-check, rust-compile, rust-test, rust-run

extern crate image;
extern crate rand;

mod vector;
mod color;
mod triangle;
mod surface_element;
mod sphere;
mod camera;
mod scene;
mod rendering;
mod material;
mod ray;

use vector::*;
use color::*;
use triangle::*;
use sphere::*;
use camera::*;
use scene::*;
use rendering::*;
use material::*;

fn main() {
    println!("rendering...");

    let camera = Camera {
	position: Vector{x: 0.001f32, y: -0.801f32, z: 1.40001f32},
	forward:  Vector{x: 0f32, y: 1f32, z: 0f32},
	right:    Vector{x: 1f32, y: 0f32, z: 0f32},
	up:       Vector{x: 0f32, y: 0f32, z: 1f32},
    };

    let mut scene = Scene {
	triangles: Vec::new(),
	sphere: Sphere {
	    position: Vector {
		x: 0f32,
		y: 0.7f32,
		z: 1.5f32
	    },
	    radius: 0.05f32,
	    color: Color {
		r: 1f32,
		g: 1f32,
		b: 1f32,
	    },
	},
    };

    //floor
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 0f32},
	v1:   Vector{x: -2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: -2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.3f32,
		g: 0.3f32,
		b: 0.3f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.3f32,
		g: 0.3f32,
		b: 0.3f32,
	    },
	},
    });

    //ceiling
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: -2f32, z: 0f32},
	v2:   Vector{x: -2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 2f32, z: 0f32},
	v2:   Vector{x: 2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	},
    });


    //right wall
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 0f32, z: -2f32},
	v2:   Vector{x: 0f32, y: -2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.1f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 0f32, z: 2f32},
	v2:   Vector{x: 0f32, y: 2f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.8f32,
		b: 0.1f32,
	    },
	},
    });

    //left wall
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: -2f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: -2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.1f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 2f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.1f32,
		g: 0.8f32,
		b: 0.8f32,
	    },
	},
    });

    //far wall
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: 1f32, z: 0f32},
	v1:   Vector{x: 0f32, y: 0f32, z: 2f32},
	v2:   Vector{x: -2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.1f32,
		b: 0.8f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: 1f32, z: 2f32},
	v1:   Vector{x: 0f32, y: 0f32, z: -2f32},
	v2:   Vector{x: 2f32, y: 0f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.8f32,
		g: 0.1f32,
		b: 0.8f32,
	    },
	},
    });

    //wall behind camera
    scene.triangles.push(Triangle {
	base: Vector{x: 1f32, y: -1f32, z: 0f32},
	v1:   Vector{x: -2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: 2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.5f32,
		g: 0.5f32,
		b: 0.5f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: -1f32, y: -1f32, z: 2f32},
	v1:   Vector{x: 2f32, y: 0f32, z: 0f32},
	v2:   Vector{x: 0f32, y: 0f32, z: -2f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.5f32,
		g: 0.5f32,
		b: 0.5f32,
	    },
	},
    });

    //occluder
    scene.triangles.push(Triangle {
	base: Vector{x: 0f32, y: 0.4f32, z: 1.2f32},
	v1:   Vector{x: 0.5f32, y: 1f32, z: 0f32},
	v2:   Vector{x: -1f32, y: 1f32, z: 0.4f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: 0f32, y: 0.4f32, z: 1.2f32},
	v1:   Vector{x: 0f32, y: 1f32, z: -0.5f32},
	v2:   Vector{x: 0.5f32, y: 1f32, z: 0f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	},
    });
    scene.triangles.push(Triangle {
	base: Vector{x: 0f32, y: 0.4f32, z: 1.2f32},
	v1:   Vector{x: -1f32, y: 1f32, z: 0.4f32},
	v2:   Vector{x: 0f32, y: 1f32, z: -0.5f32},
	material: Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	},
    });

    
    let width = 16 * 20;
    let height = 9 * 20;
    
    let mut rendering = Rendering::rendering(width, height);

    for py in 0 .. height {
	println!("{} out of {}...", py, height);
	for px in 0 .. width {
	    let pixel = rendering.get_mut_pixel(px, py);

	    let x = 2f32 * ((px as f32) - (width  as f32 * 0.5f32)) / (height as f32);	    // using height to keep aspect ratio
	    let y = -2f32 * ((py as f32) - (height as f32 * 0.5f32)) / (height as f32);

	    let ray = camera.shoot_ray(x, y);

	    let num_samples = 25;
	    let mut accumulator = BLACK;
	    for _ in 0 .. num_samples {
		accumulator = accumulator + scene.trace_ray(ray, 5);
	    }
	    
	    *pixel = accumulator * (1f32 / num_samples as f32);
	}
    }

    println!("{:#?}", rendering.pixels[width * height - 300]);
    println!("{:#?}", rendering.pixels[width * height - 400]);
    println!("{:#?}", rendering.pixels[width * height - 500]);
    println!("{:#?}", rendering.pixels[width * height - 600]);
    println!("{:#?}", rendering.pixels[width * height - 700]);
    
    
    rendering.apply_gamma(0.2f32);
    
    println!("saving...");
    
    rendering.save();
}
