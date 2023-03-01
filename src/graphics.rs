use miniquad::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vec2 {
	x: f32,
	y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vertex {
	pos: Vec2,
	uv: Vec2,
}

pub struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
}

impl Stage {
	pub fn new(ctx: &mut Context, texture_atlas: Texture) -> Self {
		let vert = |x, y, u, v| Vertex {
			pos: Vec2 { x, y },
			uv: Vec2 { x: u, y: v },
		};
		let shader = Shader::new(
			ctx,
			include_str!("../shaders/vert.glsl"),
			include_str!("../shaders/frag.glsl"),
			ShaderMeta {
				images: vec!["texture_atlas".to_string()],
				uniforms: UniformBlockLayout { uniforms: vec![] },
			},
		)
		.unwrap();
		let pipeline = Pipeline::new(
			ctx,
			&[BufferLayout::default()],
			&[
				VertexAttribute::new("pos", VertexFormat::Float2),
				VertexAttribute::new("uv", VertexFormat::Float2),
			],
			shader,
		);
		let bindings = Bindings {
			vertex_buffers: vec![Buffer::immutable(
				ctx,
				BufferType::VertexBuffer,
				&[
					vert(-1., -1., 0., 0.),
					vert(1., -1., 1., 0.),
					vert(1., 1., 1., 1.),
					vert(-1., 1., 0., 1.),
				],
			)],
			index_buffer: Buffer::immutable::<u16>(
				ctx,
				BufferType::IndexBuffer,
				&[0, 1, 2, 0, 2, 3],
			),
			images: vec![texture_atlas],
		};
		Self { pipeline, bindings }
	}

	pub fn render(&mut self, ctx: &mut Context) {
		ctx.begin_default_pass(Default::default());
		ctx.apply_pipeline(&self.pipeline);
		ctx.apply_bindings(&self.bindings);
		ctx.draw(0, 6, 1);
		ctx.end_render_pass();
		ctx.commit_frame();
	}
}
