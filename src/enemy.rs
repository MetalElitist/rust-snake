use ggez::mint::{Point2};
use ggez::graphics::{Mesh, draw};
use ggez::Context;
use ggez::nalgebra as na;

use crate::{MainState, cellsize, cellcols, cellrows};
use crate::create_mesh::create_mesh;
use crate::snake::MovingDir;

use rand::Rng;

pub struct Enemy {
	pub pos: Point2<i32>,
	mesh: Mesh,
	movingrate: u8,
	update_count: u32,
	occupied_cells: [Point2<i32>;2],
}
use std::rc::Rc;

impl Enemy {
	pub fn new(ctx: &mut Context) -> Self {
		Enemy {
			pos: MainState::random_position(),
			mesh: create_mesh(ctx, "/images/enemies/rust1.png", cellsize as u16*2, cellsize as u16),
			movingrate: 150,
			update_count: 0,
			occupied_cells: [Point2::from_slice(&[0, 0]);2]
		}
	}


	pub fn update(&mut self, occupied_cells: &[Point2<i32>]) {
		if self.update_count % self.movingrate as u32 == 0 {
			self.move_in_dir(MovingDir::random(), occupied_cells);
		}
		self.update_count += 1;
	}

	pub fn draw(&self, ctx: &mut Context) {
		draw(ctx, &self.mesh, (na::Point2::<f32>::new(self.pos.x as f32 * cellsize as f32 + cellsize as f32, self.pos.y as f32 * cellsize as f32 + cellsize as f32/2.0),));
	}

	pub fn overlapping(&self, point: Point2<i32>) -> bool {
		if (self.pos.x == point.x && self.pos.y == point.y) || (self.pos.x + 1 == point.x && self.pos.y == point.y) {
			true
		} else {
			false
		}
	}

	pub fn move_in_dir(&mut self, dir: MovingDir, occupied_cells: &[Point2<i32>]) {
		let dir = dir.to_vel();
		let newpos = Point2::from_slice(&[self.pos.x + dir.x, self.pos.y + dir.y]);
		if newpos.x < 0 || newpos.y < 0 || newpos.x + 1 >= cellcols as i32 || newpos.y + 1 >= cellrows as i32 {
			return
		}
		for oc in occupied_cells {
			if self.overlapping(*oc) {
				continue;
			}
			if (newpos.x == oc.x && newpos.y == oc.y) || (newpos.x+1 == oc.x && newpos.y == oc.y) {
				return
			}
		}
		self.pos = newpos;
	}

	pub fn occupied_cells(&mut self) -> &[Point2<i32>] {
		self.occupied_cells[0] = self.pos;
		self.occupied_cells[1] = Point2::from_slice(&[self.pos.x+1, self.pos.y]);
		&self.occupied_cells
	}
}