use miniquad::*;

#[repr(C)]
struct Vec2 {
	x: f32,
	y: f32,
}

#[repr(C)]
struct Color {
	r: f32,
	g: f32,
	b: f32,
}

#[repr(C)]
struct Vertex {
	pos: Vec2,
	color: Color,
}

pub struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
}

impl Stage {
	pub fn new(ctx: &mut Context) -> Self {
		let vert = |x, y, r, g, b| Vertex {
			pos: Vec2 { x, y },
			color: Color { r, g, b },
		};
		let bindings = Bindings {
			vertex_buffers: vec![Buffer::immutable(
				ctx,
				BufferType::VertexBuffer,
				&[
					vert(0., 0.5, 1., 0., 0.),
					vert(-0.5, -0.5, 0., 1., 0.),
					vert(0.5, -0.5, 0., 0., 1.),
				],
			)],
			index_buffer: Buffer::immutable(ctx, BufferType::IndexBuffer, &[0, 1, 2]),
			images: vec![],
		};
		let shader = Shader::new(
			ctx,
			include_str!("../shaders/vert.glsl"),
			include_str!("../shaders/frag.glsl"),
			ShaderMeta {
				images: vec![],
				uniforms: UniformBlockLayout { uniforms: vec![] },
			},
		)
		.unwrap();
		let pipeline = Pipeline::new(
			ctx,
			&[BufferLayout::default()],
			&[
				VertexAttribute::new("pos", VertexFormat::Float2),
				VertexAttribute::new("color", VertexFormat::Float3),
			],
			shader,
		);
		Self { pipeline, bindings }
	}
	pub fn render(&mut self, ctx: &mut Context) {
		ctx.begin_default_pass(Default::default());
		ctx.apply_pipeline(&self.pipeline);
		ctx.apply_bindings(&self.bindings);
		ctx.draw(0, 3, 1);
		ctx.end_render_pass();
		ctx.commit_frame();
	}
}
