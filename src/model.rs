use std::fs;

use crate::vector::*;
use crate::triangle::*;
use crate::material::*;
use crate::color::*;

#[derive(Clone,Debug)]
pub struct RawModel {
    vertices: Vec<Vector>,
    triangle_indices: Vec<(usize, usize, usize)>,
}

impl RawModel {
    pub fn load_to_raw(model_name: &str) -> RawModel {
	println!("loading: {}", model_name);

	let mut relative_path = String::from("models/");
	relative_path.push_str(model_name);

	let mut raw_model = RawModel {
	    vertices: Vec::new(),
	    triangle_indices: Vec::new(),
	};

	{ // load vertices
	    let mut vertices_path = relative_path.clone();
	    vertices_path.push_str("/vertices.txt");

	    let vertices_file = fs::read_to_string(vertices_path.as_str()).expect(format!("couldn't read the file {}", vertices_path).as_str());

	    for line in vertices_file.lines() {
		let mut words = line.split_whitespace();
		let x_string = words.next().expect("couldn't read x");
		let y_string = words.next().expect("couldn't read y");
		let z_string = words.next().expect("couldn't read z");

		let x: f32 = x_string.parse().expect("couldn't parse x");
		let y: f32 = y_string.parse().expect("couldn't parse y");
		let z: f32 = z_string.parse().expect("couldn't parse z");

		// TEMP y and z flipped TEMP <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
		let new_vertex = Vector {
		    x: x,
		    y: -z,
		    z: y,
		};

		raw_model.vertices.push(new_vertex);
	    }
	}
	
	{ // load triangle indices
	    let mut triangles_path = relative_path.clone();
	    triangles_path.push_str("/triangles.txt");

	    let triangles_file = fs::read_to_string(triangles_path.as_str()).expect(format!("couldn't read the file {}", triangles_path).as_str());

	    for line in triangles_file.lines() {
		let mut words = line.split_whitespace();
		let n1_string = words.next().expect("couldn't read n1");
		let n2_string = words.next().expect("couldn't read n2");
		let n3_string = words.next().expect("couldn't read n3");

		let n1: usize = n1_string.parse().expect("couldn't parse n1");
		let n2: usize = n2_string.parse().expect("couldn't parse n2");
		let n3: usize = n3_string.parse().expect("couldn't parse n3");

		raw_model.triangle_indices.push((n1,n2,n3));
	    }
	}
	
	raw_model
    }
}

pub struct Model {
    pub triangles: Vec<Triangle>,
}

impl Model {
    pub fn from_raw(raw_model: RawModel) -> Model {
	let mut model = Model {
	    triangles: Vec::new(),
	};

	let default_material = Material {
	    diffuse_color: Color {
		r: 0.9f32,
		g: 0.9f32,
		b: 0.9f32,
	    },
	};
	
	for triangle_n123 in raw_model.triangle_indices {
	    let (n1, n2, n3) = triangle_n123;

	    let v1_raw = raw_model.vertices[n1];
	    let v2_raw = raw_model.vertices[n2];
	    let v3_raw = raw_model.vertices[n3];

	    let base = v1_raw;
	    let v1 = v2_raw - v1_raw;
	    let v2 = v3_raw - v1_raw;

	    let triangle = Triangle {
		base: base,
		v1: v1,
		v2: v2,
		material: default_material,
	    };

	    model.triangles.push(triangle);
	}

	model
    }
}
