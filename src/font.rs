use crate::graphics::{Vec2, Vertex};

///Stores attributes required to render each Glyph
///Dimensions must be specified in a top-left coordinate system with `(0, 0)` at top-left and `(1, 1)` at bottom-right
#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub size: Vec2,
	pub pos: Vec2,
	pub uv_pos: Vec2,
	pub uv_size: Vec2,
}

impl Glyph {
	pub fn as_vertices(&self) -> [Vertex; 4] {
		let glyph_width = self.size.x;
		let glyph_height = self.size.y;
		let glyph_x = self.pos.x;
		let glyph_y = self.pos.y;

		let uv_x = self.uv_pos.x;
		let uv_y = self.uv_pos.y;
		let uv_width = self.uv_size.x;
		let uv_height = self.uv_size.y;

		let lerp = |a: f32, b: f32, t: f32| a * (1.0 - t) + b * t;

		let top_left = Vertex {
			pos: Vec2 {
				x: lerp(-1.0, 1.0, glyph_x),
				y: lerp(1.0, -1.0, glyph_y),
			},
			uv: Vec2 {
				x: uv_x,
				y: lerp(1.0 - uv_y, 1.0 - uv_y - uv_height, glyph_y),
			},
		};
		let top_right = Vertex {
			pos: Vec2 {
				x: lerp(-1.0, 1.0, glyph_x + glyph_width),
				y: lerp(1.0, -1.0, glyph_y),
			},
			uv: Vec2 {
				x: uv_x + uv_width,
				y: lerp(1.0 - uv_y, 1.0 - uv_y - uv_height, glyph_y),
			},
		};
		let bottom_left = Vertex {
			pos: Vec2 {
				x: lerp(-1.0, 1.0, glyph_x),
				y: lerp(1.0, -1.0, glyph_y + glyph_height),
			},
			uv: Vec2 {
				x: uv_x,
				y: lerp(1.0 - uv_y, 1.0 - uv_y - uv_height, glyph_y + glyph_height),
			},
		};
		let bottom_right = Vertex {
			pos: Vec2 {
				x: lerp(-1.0, 1.0, glyph_x + glyph_width),
				y: lerp(1.0, -1.0, glyph_y + glyph_height),
			},
			uv: Vec2 {
				x: uv_x + uv_width,
				y: lerp(1.0 - uv_y, 1.0 - uv_y - uv_height, glyph_y + glyph_height),
			},
		};

		[bottom_left, bottom_right, top_right, top_left]
	}
}

/// Reads pbm files from the given file_path
/// Return a tuple with following parameters of the image `(width, height, vector with pixel values)`
/// Comments are also not supported
pub fn parse_pgm(file_data: &str) -> Option<(u32, u32, Vec<u8>)> {
	let mut metadata = file_data.chars();
	if metadata.next_chunk::<2>().ok()?.iter().collect::<String>() != *"P2" {
		return None;
	}
	metadata.next()?;
	let mut dimensions = Vec::<u32>::with_capacity(2);
	let mut pixel_data = Vec::new();
	for (line_number, line) in file_data.lines().enumerate() {
		if line_number == 0 || line_number == 3 || line.trim_start().starts_with('#') {
			continue;
		}
		if dimensions.is_empty() {
			dimensions = line
				.split_whitespace()
				.map(|i| i.parse().unwrap())
				.collect();
			assert!(dimensions.len() == 2 && dimensions[0] > 0 && dimensions[1] > 0);
			pixel_data.reserve(dimensions.iter().product::<u32>() as usize);
		} else {
			pixel_data.push(line.parse().unwrap());
		}
	}
	assert_eq!(dimensions.iter().product::<u32>(), pixel_data.len() as u32);
	let (width, height) = (dimensions[0], dimensions[1]);
	// Flip the image vertically
	let mut pixel_data_flipped = Vec::with_capacity(pixel_data.len());
	for i in (0..height).rev() {
		for j in 0..width {
			pixel_data_flipped.push(pixel_data[(width * i + j) as usize]);
		}
	}
	Some((width, height, pixel_data_flipped))
}
