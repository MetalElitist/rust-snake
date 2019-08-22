use ggez;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context};
use ggez::event::{KeyCode};
use std::collections::HashSet;
use ggez::graphics;
use crate::cellsize;
use na::RealField;

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
}

pub struct Snake {
	pub parts: Vec<Point2<i32>>,
	dir: MovingDir,
	growing: bool,
	changingdir: MovingDir,
	partsmesh: graphics::Mesh,
	headmesh: graphics::Mesh,
	partsmesh_direct: graphics::Mesh,
	partsmesh_angle: graphics::Mesh,
	partsmesh_tail: graphics::Mesh,
}

impl Snake {
	pub fn new(ctx: &mut Context) -> Self {
		Snake {
			parts: vec![Point2::from_slice(&[1,20]), Point2::from_slice(&[1,21]), Point2::from_slice(&[1,22])],
			dir: MovingDir::EAST,
			changingdir: MovingDir::EAST,
			growing: false,
			partsmesh: graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(),
				graphics::Rect::new(0.0, 0.0, cellsize as f32, cellsize as f32), 
				graphics::Color::new(0.9, 0.05, 0.05, 1.0)
			).unwrap(),
			headmesh: Snake::create_mesh(ctx, "/images/0x0.png"),
			partsmesh_direct: Snake::create_mesh(ctx, "/images/0x3.png"),
			partsmesh_angle: Snake::create_mesh(ctx, "/images/0x2.png"),
			partsmesh_tail: Snake::create_mesh(ctx, "/images/0x1.png"),
		}
	}

	pub fn create_mesh(ctx: &mut Context, texture_path: &str) -> graphics::Mesh {
		let triangle_verts = vec![
			graphics::Vertex {
				pos: [cellsize as f32/2.0, cellsize as f32/2.0],
				uv: [1.0, 1.0],
				color: [1.0, 1.0, 1.0, 1.0],
			},
			graphics::Vertex {
				pos: [-(cellsize as f32/2.0), cellsize as f32/2.0],
				uv: [0.0, 1.0],
				color: [1.0, 1.0, 1.0, 1.0],
			},
			graphics::Vertex {
				pos: [-(cellsize as f32/2.0), -(cellsize as f32/2.0)],
				uv: [0.0, 0.0],
				color: [1.0, 1.0, 1.0, 1.0],
			},
			graphics::Vertex {
				pos: [cellsize as f32/2.0, -(cellsize as f32/2.0)],
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

	pub fn out_of_bounds(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
		for p in &self.parts {
			if p.x < left || p.y < top || p.x > right || p.y > bottom {
				return true
			}
		}
		false
	}

	pub fn colliding_with_food(&self, food: &Point2<i32>) -> bool {
		if self.parts[0] == *food {
			true
		} else {
			false
		}
	}

	pub fn eating_itself(&self) -> bool {
		let headposition = self.parts[0];
		for p in &self.parts[1..] {
			if *p == headposition {
				return true
			}
		}
		false
	}

	pub fn overlapping(&self, point: Point2<i32>) -> bool {
		for p in &self.parts {
			if *p == point {
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
		self.dir = self.changingdir;
		let dir = self.dir.to_vel();
		let newhead = Point2::from_slice(&[self.parts[0].x + self.dir.to_vel().x, self.parts[0].y + self.dir.to_vel().y]);
		self.parts.insert(0, newhead);
		if !self.growing {
			self.parts.pop();
		} else {
			self.growing = false;
		}
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

	pub fn draw(&mut self, ctx: &mut Context) {
		let headpart = self.parts[0];
		let drawpoint = na::Point2::<f32>::new(headpart.x as f32 * cellsize as f32 + cellsize as f32/2.0, headpart.y as f32 * cellsize as f32 + cellsize as f32/2.0);
		let mut head_draw_params = graphics::DrawParam::new();
		head_draw_params.rotation(self.dir.rotation());
		head_draw_params.dest(drawpoint);
		graphics::draw(ctx, &self.headmesh, (drawpoint, self.dir.rotation(), graphics::WHITE));

		for (i, p) in self.parts[1..].iter().enumerate() {

			let drawpoint = na::Point2::<f32>::new(p.x as f32 * cellsize as f32 + cellsize as f32/2.0, p.y as f32 * cellsize as f32 + cellsize as f32/2.0);
			let mut parts_draw_param = graphics::DrawParam::new();

			let mesh;
			let rotation;
			if i + 2 < self.parts.len() {
				let part_angle_info = Snake::angle_rotation_info(&self.parts[i+2], p, &self.parts[i]);
				if part_angle_info.0 {
					mesh = &self.partsmesh_angle;
					rotation = MovingDir::part_dir(p, &self.parts[i]).rotation() + part_angle_info.1;
				} else {
					mesh = &self.partsmesh_direct;
					rotation = MovingDir::part_dir(p, &self.parts[i]).rotation();
				}
			} else {
				mesh = &self.partsmesh_tail;
				rotation = MovingDir::part_dir(p, &self.parts[i]).rotation() + f32::pi();
			}
			graphics::draw(ctx, mesh, (drawpoint, rotation, graphics::WHITE));
		}
	}
}