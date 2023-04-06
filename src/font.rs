use crate::graphics::{Vec2, Vertex};

const TEXTURE_ATLAS_SIZE: Vec2 = Vec2 { x: 19., y: 5. };

#[derive(Debug)]
pub struct CharacterGrid {
	font_size: f32,
	window_size: (f32, f32),
	values: Vec<char>,
}
impl CharacterGrid {
	pub fn new(font_size: f32, window_size: (f32, f32)) -> Self {
		let grid = Self {
			font_size,
			window_size,
			values: Vec::new(),
		};
		let grid_size = grid.size();
		let values = Vec::with_capacity(grid_size.0 * grid_size.1);
		Self { values, ..grid }
	}
	pub fn insert_text(&mut self, text: &str) {
		text.chars()
			.for_each(|char| self.values.push(char));
	}
	#[inline]
	pub fn size(&self) -> (usize, usize) {
		(
			(self.window_size.0 / (self.font_size * 0.5)) as usize,
			(self.window_size.1 / self.font_size) as usize,
		)
	}
	pub fn glyphs(&self) -> impl Iterator<Item = Glyph> + '_ {
		let (grid_width, grid_height) = self.size();
		let (grid_width, grid_height) = (grid_width as f32, grid_height as f32);
		let viewport_size = Vec2::new(
			(grid_width * self.font_size * 0.5) / self.window_size.0,
			(grid_height * self.font_size) / self.window_size.1,
		);
		let offset = Vec2::new((1. - viewport_size.x) / 2., (1. - viewport_size.y) / 2.);
		let glyph_size = Vec2::new(viewport_size.x / grid_width, viewport_size.y / grid_height);
		let uv_size = Vec2::new(TEXTURE_ATLAS_SIZE.x.recip(), TEXTURE_ATLAS_SIZE.y.recip());
		let with_offset = move |x: f32, y: f32| Vec2::new(x + offset.x, y + offset.y);
		(0..self.values.len()).map(move |k| {
			let char = self.values.get(k).unwrap_or(&'\0');
			let i = k % grid_width as usize;
			let j = k / grid_width as usize;
			let uv_index = if (' '..='~').contains(char) {
				let char_val = *char as u8 - b' ';
				(
					char_val % TEXTURE_ATLAS_SIZE.x as u8,
					char_val / TEXTURE_ATLAS_SIZE.x as u8,
				)
			} else {
				(0, 0)
			};
			let uv_pos = Vec2::new(uv_index.0 as f32 * uv_size.x, uv_index.1 as f32 * uv_size.y);
			Glyph {
				size: glyph_size,
				pos: with_offset(i as f32 * glyph_size.x, j as f32 * glyph_size.y),
				uv_size,
				uv_pos,
			}
		})
	}
}

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

		let lerp = |a: f32, b: f32, t: f32| a * (1. - t) + b * t;

		let top_left = Vertex {
			pos: Vec2 {
				x: lerp(-1., 1., glyph_x),
				y: lerp(1., -1., glyph_y),
			},
			uv: Vec2 {
				x: uv_x,
				y: lerp(1., 0., uv_y),
			},
		};
		let top_right = Vertex {
			pos: Vec2 {
				x: lerp(-1., 1., glyph_x + glyph_width),
				y: lerp(1., -1., glyph_y),
			},
			uv: Vec2 {
				x: uv_x + uv_width,
				y: lerp(1., 0., uv_y),
			},
		};
		let bottom_left = Vertex {
			pos: Vec2 {
				x: lerp(-1., 1., glyph_x),
				y: lerp(1., -1., glyph_y + glyph_height),
			},
			uv: Vec2 {
				x: uv_x,
				y: lerp(1., 0., uv_y + uv_height),
			},
		};
		let bottom_right = Vertex {
			pos: Vec2 {
				x: lerp(-1., 1., glyph_x + glyph_width),
				y: lerp(1., -1., glyph_y + glyph_height),
			},
			uv: Vec2 {
				x: uv_x + uv_width,
				y: lerp(1., 0., uv_y + uv_height),
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
