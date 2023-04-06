#![feature(vec_push_within_capacity)]
#![feature(iter_next_chunk)]
use miniquad::*;
mod font;
mod graphics;
use font::CharacterGrid;
use graphics::Stage;

struct Window {
	stage: Stage,
	grid: CharacterGrid,
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
		let text_size = 20.;
		Self {
			stage: Stage::new(ctx, texture_atlas),
			grid: CharacterGrid::new(text_size, ctx.screen_size()),
		}
	}
	fn render(&mut self, ctx: &mut Context) {
		self.stage.render(ctx, self.grid.clone());
	}
}

impl EventHandler for Window {
	fn draw(&mut self, ctx: &mut Context) {
		self.render(ctx);
	}
	fn char_event(&mut self, _ctx: &mut Context, character: char, keymods: KeyMods, _repeat: bool) {
		if keymods.ctrl {
			if let 'w' = character {
				self.grid.delete_word()
			}
			return;
		}
		match character {
			//Backspace
			'\u{f02a}' => self.grid.pop(),
			//Return
			'\n' | '\u{f028}' => self.grid.fill_line(),
			_ => {
				if character.is_alphanumeric() || character.is_whitespace() {
					self.grid.insert_text(character.to_string().as_str());
				}
			}
		}
	}
	fn update(&mut self, _ctx: &mut Context) {}
}

fn main() {
	miniquad::start(conf::Conf::default(), |ctx| Box::new(Window::new(ctx)));
}
