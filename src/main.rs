use miniquad::*;
mod graphics;

struct Window {
	stage: graphics::Stage,
}

impl Window {
	fn new(ctx: &mut Context) -> Self {
		Self {
			stage: graphics::Stage::new(ctx),
		}
	}
}
impl EventHandler for Window {
	fn draw(&mut self, ctx: &mut Context) {
		self.stage.render(ctx);
	}
	fn update(&mut self, _ctx: &mut Context) {}
}

fn main() {
	miniquad::start(conf::Conf::default(), |ctx| Box::new(Window::new(ctx)));
}
