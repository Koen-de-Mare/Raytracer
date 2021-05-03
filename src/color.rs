use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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
    pub fn quantise(self) -> image::Rgb<u8> {
	let r: u8 = (self.r * 255f32).min(255f32).max(0f32) as u8;
	let g: u8 = (self.g * 255f32).min(255f32).max(0f32) as u8;
	let b: u8 = (self.b * 255f32).min(255f32).max(0f32) as u8;
	image::Rgb([r, g, b])
    }
}

pub const BLACK: Color = Color {
    r: 0f32,
    g: 0f32,
    b: 0f32,
};
