use miniquad::*;

struct Window {}

impl Window {
	fn new() -> Self {
		Self {}
	}
}
impl EventHandler for Window {
	fn draw(&mut self, _ctx: &mut Context) {}
	fn update(&mut self, _ctx: &mut Context) {}
}

fn main() {
	miniquad::start(conf::Conf::default(), |_ctx| Box::new(Window::new()));
}

