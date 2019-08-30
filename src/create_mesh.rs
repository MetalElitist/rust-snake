use ggez::graphics;
use ggez::{Context};

use crate::cellsize;

pub fn create_mesh(ctx: &mut Context, texture_path: &str, width: u16, height: u16) -> graphics::Mesh {
	let triangle_verts = vec![
		graphics::Vertex {
			pos: [width as f32/2.0, height as f32/2.0],
			uv: [1.0, 1.0],
			color: [1.0, 1.0, 1.0, 1.0],
		},
		graphics::Vertex {
			pos: [-(width as f32/2.0), height as f32/2.0],
			uv: [0.0, 1.0],
			color: [1.0, 1.0, 1.0, 1.0],
		},
		graphics::Vertex {
			pos: [-(width as f32/2.0), -(height as f32/2.0)],
			uv: [0.0, 0.0],
			color: [1.0, 1.0, 1.0, 1.0],
		},
		graphics::Vertex {
			pos: [width as f32/2.0, -(height as f32/2.0)],
			uv: [1.0, 0.0],
			color: [1.0, 1.0, 1.0, 1.0],
		},
	];


	let triangle_indices = vec![0,1,2,2,3,0];
	let img = graphics::Image::new(ctx, texture_path).ok();
	graphics::Mesh::from_raw(
		ctx,
		&triangle_verts,
		&triangle_indices,
		img,
	).unwrap()
}
