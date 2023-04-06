#![feature(vec_push_within_capacity)]
#![feature(iter_next_chunk)]
use miniquad::*;
mod font;
mod graphics;
use font::CharacterGrid;
use graphics::Stage;

struct Window {
	stage: Stage,
}

impl Window {
	fn new(ctx: &mut Context) -> Self {
		let (width, height, pixel_data) =
			font::parse_pgm(include_str!("../res/font_atlas.pgm")).unwrap();
		let texture_atlas = Texture::new(
			ctx,
			TextureAccess::Static,
			Some(&pixel_data),
			TextureParams {
				format: TextureFormat::Alpha,
				width,
				height,
				..Default::default()
			},
		);
		Self {
			stage: Stage::new(ctx, texture_atlas),
		}
	}
}

impl EventHandler for Window {
	fn draw(&mut self, ctx: &mut Context) {
		let mut grid = CharacterGrid::new(40., ctx.screen_size());
		grid.insert_text("The quick brown fox jumped over the lazy dogs");
		self.stage.render(ctx, grid);
	}
	fn update(&mut self, _ctx: &mut Context) {}
}

fn main() {
	miniquad::start(conf::Conf::default(), |ctx| Box::new(Window::new(ctx)));
}
