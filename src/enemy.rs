use ggez::mint::{Point2};
use ggez::graphics::{Mesh, draw};
use ggez::Context;
use ggez::nalgebra as na;

use crate::{MainState, cellsize, cellcols, cellrows};
use crate::create_mesh::create_mesh;
use crate::snake::MovingDir;
use crate::cellrect::CellRect;

use rand::Rng;

pub struct Enemy {
	pub rect: CellRect,
	mesh: Mesh,
	movingrate: u8,
	update_count: u32,
	occupied_cells: [Point2<i32>;2],
}
use std::rc::Rc;

impl Enemy {
	pub fn new(ctx: &mut Context) -> Self {
		Enemy {
			rect: CellRect {pos: MainState::random_position(), w: 2, h: 2},
			mesh: create_mesh(ctx, "/images/enemies/rust1.png", cellsize as u16*2, cellsize as u16),
			movingrate: 150,
			update_count: 0,
			occupied_cells: [Point2::from_slice(&[0, 0]);2]
		}
	}


	pub fn update(&mut self, occupied_rects: &[CellRect]) {
		if self.update_count % self.movingrate as u32 == 0 {
			self.move_in_dir(MovingDir::random(), occupied_rects);
		}
		self.update_count += 1;
	}

	pub fn draw(&self, ctx: &mut Context) {
		draw(ctx, &self.mesh, (na::Point2::<f32>::new(self.rect.pos.x as f32 * cellsize as f32 + cellsize as f32, self.rect.pos.y as f32 * cellsize as f32 + cellsize as f32/2.0),));
	}

	pub fn overlapping(&self, rect: &CellRect) -> bool {
		if rect.overlapping(&self.rect) {
			true
		} else {
			false
		}
	}

	pub fn move_in_dir(&mut self, dir: MovingDir, occupied_rects: &[CellRect]) {
		let dir = dir.to_vel();
		let newpos = Point2::from_slice(&[self.rect.pos.x + dir.x, self.rect.pos.y + dir.y]);
		if newpos.x < 0 || newpos.y < 0 || newpos.x + 1 >= cellcols as i32 || newpos.y + 1 >= cellrows as i32 {
			return
		}
		for or in occupied_rects {
			if self.overlapping(or) {
				continue;
			}
			if (newpos.x == or.pos.x && newpos.y == or.pos.y) || (newpos.x+1 == or.pos.x && newpos.y == or.pos.y) {
				return
			}
		}
		self.rect.pos = newpos;
	}

}