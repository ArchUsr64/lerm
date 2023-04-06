use crate::font::CharacterGrid;
use miniquad::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}
impl Vec2 {
	pub(crate) fn new(x: f32, y: f32) -> Self {
		Vec2 { x, y }
	}
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
	pub pos: Vec2,
	pub uv: Vec2,
}

pub struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
	vertex_buffer: Vec<Vertex>,
}

impl Stage {
	pub fn new(ctx: &mut Context, texture_atlas: Texture) -> Self {
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
			vertex_buffers: vec![Buffer::immutable::<Vertex>(
				ctx,
				BufferType::VertexBuffer,
				&[],
			)],
			index_buffer: Buffer::immutable::<u16>(ctx, BufferType::IndexBuffer, &[]),
			images: vec![texture_atlas],
		};
		Self {
			pipeline,
			bindings,
			vertex_buffer: Vec::new(),
		}
	}

	pub fn render(&mut self, ctx: &mut Context, character_grid: CharacterGrid) {
		ctx.begin_default_pass(Default::default());
		ctx.apply_pipeline(&self.pipeline);
		self.bindings.vertex_buffers[0].delete();
		self.vertex_buffer = Vec::new();
		character_grid
			.glyphs()
			.for_each(|glyph| self.vertex_buffer.append(&mut glyph.as_vertices().to_vec()));
		self.bindings.vertex_buffers = vec![Buffer::immutable(
			ctx,
			BufferType::VertexBuffer,
			&self.vertex_buffer,
		)];
		self.bindings.index_buffer.delete();
		let index_buffer: Vec<_> = (0..self.vertex_buffer.len() as u16 / 4)
			.flat_map(|i| [0, 1, 2, 0, 2, 3].iter().map(move |j| 4 * i + j))
			.collect();
		self.bindings.index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &index_buffer);
		ctx.apply_bindings(&self.bindings);
		ctx.draw(0, self.vertex_buffer.len() as i32 * 6 / 4, 1);
		ctx.end_render_pass();
		ctx.commit_frame();
	}
}
