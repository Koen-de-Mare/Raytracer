use image::ImageBuffer;

use crate::color::*;

pub struct Rendering {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Rendering {
    pub fn rendering(width: usize, height: usize) -> Rendering {
	let mut rendering = Rendering {
	    width: width,
	    height: height,
	    pixels: Vec::new(),
	};

	rendering.pixels.resize(width * height, BLACK);

	rendering
    }

    pub fn get_mut_pixel(&mut self, px: usize, py: usize) -> &mut Color {
	assert!(self.pixels.len() == self.width * self.height);

	assert!(px < self.width);
	assert!(py < self.height);
	
	let n = px + py * self.width;

	self.pixels.get_mut(n).unwrap()
    }
    
    pub fn save(self) {
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
