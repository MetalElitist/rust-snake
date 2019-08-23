use ggez::mint::{Point2};
use ggez::graphics::{Mesh, draw};
use ggez::Context;
use ggez::nalgebra as na;

use crate::{MainState, cellsize, cellcols, cellrows};
use crate::create_mesh::create_mesh;
use crate::snake::MovingDir;

use rand::Rng;

pub struct Enemy {
	pos: Point2<i32>,
	mesh: Mesh,
	movingrate: u8,
	update_count: u32,
}

impl Enemy {
	pub fn new(ctx: &mut Context) -> Self {
		Enemy {
			pos: MainState::random_position(),
			mesh: create_mesh(ctx, "/images/enemies/rust1.png", cellsize as u16*2, cellsize as u16),
			movingrate: 10,
			update_count: 0,
		}
	}

	pub fn update(&mut self) {
		if self.update_count % self.movingrate as u32 == 0 {
			self.move_in_dir(MovingDir::random());
		}
		self.update_count += 1;
	}

	pub fn draw(&self, ctx: &mut Context) {
		draw(ctx, &self.mesh, (na::Point2::<f32>::new(self.pos.x as f32 * cellsize as f32 + cellsize as f32, self.pos.y as f32 * cellsize as f32 + cellsize as f32/2.0),));
	}

	pub fn move_in_dir(&mut self, dir: MovingDir) {
		let dir = dir.to_vel();
		let newpos = Point2::from_slice(&[self.pos.x + dir.x, self.pos.y + dir.y]);
		if newpos.x < 0 || newpos.y < 0 || newpos.x + 1 >= cellcols as i32 || newpos.y + 1 >= cellrows as i32 {
			return
		}
		self.pos = newpos;
	}
}