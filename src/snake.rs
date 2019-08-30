use ggez;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context};
use ggez::event::{KeyCode};
use std::collections::HashSet;
use ggez::graphics;
use crate::cellsize;
use na::RealField;

use crate::{GridRect,create_mesh};

use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum MovingDir {
	NORTH,
	EAST,
	SOUTH, 
	WEST
}

impl MovingDir {
	pub fn to_vel(&self) -> Point2<i32> {
		match self {
			MovingDir::NORTH => Point2::from_slice(&[0,-1]),
			MovingDir::EAST => Point2::from_slice(&[1,0]),
			MovingDir::SOUTH => Point2::from_slice(&[0,1]),
			MovingDir::WEST => Point2::from_slice(&[-1,0]),
		}
	}
	pub fn can_change_dir(self, newdir: MovingDir) -> MovingDir {
		match newdir {
			MovingDir::NORTH => {
				match self {
					MovingDir::SOUTH => self,
					_ => newdir,
				}
			},
			MovingDir::EAST => {
				match self {
					MovingDir::WEST => self,
					_ => newdir,
				}
			},
			MovingDir::SOUTH => {
				match self {
					MovingDir::NORTH => self,
					_ => newdir,
				}
			},
			MovingDir::WEST => {
				match self {
					MovingDir::EAST => self,
					_ => newdir,
				}
			},
		}
	}
	pub fn rotation(&self) -> f32 {
		match self {
			MovingDir::NORTH => 0.0,
			MovingDir::EAST => f32::two_pi()*0.25,
			MovingDir::SOUTH => f32::pi(),
			MovingDir::WEST => f32::pi()/2.0+f32::pi(),
		}
	}

	pub fn part_dir(part: &Point2<i32>, prevpart: &Point2<i32>) -> MovingDir {
		let dir = Point2::from_slice(&[part.x - prevpart.x, part.y - prevpart.y]);
		if dir.x == 1 {
			return MovingDir::EAST
		} else if dir.x == -1 {
			return MovingDir::WEST
		} else if dir.y == 1 {
			return MovingDir::SOUTH
		} else if dir.y == -1 {
			return MovingDir::NORTH
		};
		return MovingDir::EAST
	}

	pub fn random() -> MovingDir {
		match rand::thread_rng().gen_range(0, 4) {
			0 => MovingDir::EAST,
			1 => MovingDir::WEST,
			2 => MovingDir::NORTH,
			_ => MovingDir::SOUTH,
		}
	}
}

pub struct Snake {
	pub parts: Vec<GridRect>,
	dir: MovingDir,
	growing: bool,
	was_adding_part: bool,
	changingdir: MovingDir,
	update_count: u32,
	partsmesh: graphics::Mesh,
	headmesh: graphics::Mesh,
	partsmesh_direct: graphics::Mesh,
	partsmesh_angle: graphics::Mesh,
	partsmesh_tail: graphics::Mesh,
}

impl Snake {
	pub fn new(ctx: &mut Context) -> Self {
		let headmesh = create_mesh::create_mesh(ctx, "/images/snake/0x0.PNG", cellsize.into(), cellsize.into());

		Snake {
			parts: vec![GridRect {pos: Point2::from_slice(&[1,20]), w:1, h:1}, GridRect {pos: Point2::from_slice(&[1,21]), w:1, h:1}, GridRect {pos: Point2::from_slice(&[1,22]), w:1, h:1}],
			dir: MovingDir::EAST,
			was_adding_part: false,
			changingdir: MovingDir::EAST,
			growing: false,
			update_count: 0,
			partsmesh: graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(),
				graphics::Rect::new(0.0, 0.0, cellsize as f32, cellsize as f32), 
				graphics::Color::new(0.9, 0.05, 0.05, 1.0)
			).unwrap(),
			headmesh,
			partsmesh_direct: create_mesh::create_mesh(ctx, "/images/snake/0x3.PNG", cellsize.into(), cellsize.into()),
			partsmesh_angle: create_mesh::create_mesh(ctx, "/images/snake/0x2.PNG", cellsize.into(), cellsize.into()),
			partsmesh_tail: create_mesh::create_mesh(ctx, "/images/snake/0x1.PNG", cellsize.into(), cellsize.into()),
		}
	}


	pub fn out_of_bounds(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
		for p in &self.parts {
			if p.pos.x < left || p.pos.y < top || p.pos.x > right || p.pos.y > bottom {
				return true
			}
		}
		false
	}

	pub fn colliding_with_food(&self, food: &GridRect) -> bool {
		if self.parts[0].overlapping(food) {
			true
		} else {
			false
		}
	}

	pub fn eating_itself(&self) -> bool {
		let head = &self.parts[0];
		for p in &self.parts[1..] {
			if (*p).pos == head.pos {
				return true
			}
		}
		false
	}

	pub fn overlapping(&self, rect: &GridRect) -> bool {
		for p in &self.parts {
			if p.overlapping(rect) {
				return true
			}
		}
		false
	}

	pub fn angle_rotation_info(nextpart: &Point2<i32>, part: &Point2<i32>, prevpart: &Point2<i32>) -> (bool, f32) {
		if nextpart.x != prevpart.x && nextpart.y != prevpart.y {
			let side = (nextpart.x - prevpart.x) as f32 * (part.y - prevpart.y) as f32 - (nextpart.y - prevpart.y) as f32 * (part.x - prevpart.x) as f32;
			if 0.0 < side {
				(true, f32::pi())
			} else {
				(true, f32::pi()/2.0)
			}
		} else {
			(false, 0.0)
		}
	}

	pub fn grow(&mut self) {
		self.growing = true;
	}

	pub fn update(&mut self) {
		let mut last_p_pos = self.parts.last().unwrap().clone();
		if self.update_count % ((self.parts.len() as f32/80.0) + 17.0) as u32 == 0 {
			self.dir = self.changingdir;
			let dir = self.dir.to_vel();
			let newhead = GridRect {
				pos: Point2::from_slice(&[self.parts[0].pos.x + self.dir.to_vel().x, self.parts[0].pos.y + self.dir.to_vel().y]),
				w: 1, 
				h: 1,
			};
			self.parts.insert(0, newhead);
			last_p_pos = self.parts.pop().unwrap().clone();
			self.was_adding_part = false;
		}
		if self.growing {
			self.growing = false;
			self.parts.push(last_p_pos);
			self.was_adding_part = true;
		}
		self.update_count += 1;
	}

	pub fn hande_input(&mut self, keycode: &HashSet<KeyCode>) {
		// println!("{:?}", keycode);
		if keycode.contains(&KeyCode::Right) {
			self.changingdir = self.dir.can_change_dir(MovingDir::EAST)
		} else if keycode.contains(&KeyCode::Up) {
			self.changingdir = self.dir.can_change_dir(MovingDir::NORTH)
		} else if keycode.contains(&KeyCode::Left) {
			self.changingdir = self.dir.can_change_dir(MovingDir::WEST)
		} else if keycode.contains(&KeyCode::Down) {
			self.changingdir = self.dir.can_change_dir(MovingDir::SOUTH)
		}
	}

	pub fn occupied_rects(&self, mut ocvec: Vec<GridRect>) -> &[GridRect] {
		self.parts.as_slice()
	}

	pub fn draw(&mut self, ctx: &mut Context) {
		let headpart = self.parts[0].pos;
		let drawpoint = na::Point2::<f32>::new(headpart.x as f32 * cellsize as f32 + cellsize as f32/2.0, headpart.y as f32 * cellsize as f32 + cellsize as f32/2.0);
		let mut head_draw_params = graphics::DrawParam::new();
		head_draw_params.rotation(self.dir.rotation());
		head_draw_params.dest(drawpoint);
		graphics::draw(ctx, &self.headmesh, (drawpoint, self.dir.rotation(), graphics::WHITE));

		let endslice = if self.was_adding_part {self.parts.len()-1} else {self.parts.len()};
		let mut iters = 0;
		for (i, p) in self.parts[1..endslice].iter().enumerate() {

			let drawpoint = na::Point2::<f32>::new(p.pos.x as f32 * cellsize as f32 + cellsize as f32/2.0, p.pos.y as f32 * cellsize as f32 + cellsize as f32/2.0);
			let mut parts_draw_param = graphics::DrawParam::new();

			let mesh;
			let rotation;
			if i + 2 < endslice {
				let part_angle_info = Snake::angle_rotation_info(&self.parts[i+2].pos, &p.pos, &self.parts[i].pos);
				if part_angle_info.0 {
					mesh = &self.partsmesh_angle;
					rotation = MovingDir::part_dir(&p.pos, &self.parts[i].pos).rotation() + part_angle_info.1;
				} else {
					mesh = &self.partsmesh_direct;
					rotation = MovingDir::part_dir(&p.pos, &self.parts[i].pos).rotation();
				}
			} else {
				mesh = &self.partsmesh_tail;
				rotation = MovingDir::part_dir(&p.pos, &self.parts[i].pos).rotation() + f32::pi();
			}
			graphics::draw(ctx, mesh, (drawpoint, rotation, graphics::WHITE));
			iters+=1;
		}
	}
}

