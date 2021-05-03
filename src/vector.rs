use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
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
    pub fn norm2(self) -> f32 {
	dot(self, self)
    }

    pub fn norm(self) -> f32 {
	dot(self, self).sqrt()
    }
    
    pub fn normalised(self) -> Vector {
	self * (1f32 / self.norm())
    }

    pub fn is_normal(self) -> bool {
	let norm2 = self.norm2();

	norm2 < 1.0001f32 && norm2 > 0.9999f32
    }

    pub fn make_orthogonal_frame(self) -> (Vector, Vector) {
	let normalised = self.normalised();
	let vec_start = if normalised.x.abs() < 0.8f32 {
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

	let v1 = cross(normalised, vec_start).normalised();
	let v2 = cross(normalised, v1).normalised();

	assert!(dot(normalised, v1).abs() < 0.0001f32);
	assert!(dot(normalised, v2).abs() < 0.0001f32);
	assert!(dot(v1,         v2).abs() < 0.0001f32);
	
	(v1, v2)
    }
}

pub fn dot(v1: Vector, v2: Vector) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vector, v2: Vector) -> Vector {
    Vector {
	x: v1.y * v2.z - v1.z * v2.y,
	y: v1.z * v2.x - v1.x * v2.z,
	z: v1.x * v2.y - v1.y * v2.x,
    }
}
