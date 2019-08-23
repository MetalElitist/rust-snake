use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::event::{KeyCode, KeyMods};
use ggez::event;

use std::fs::{ReadDir, read_dir};
use std::path::Path;
use std::collections::LinkedList;

mod create_mesh;
mod snake;
mod enemy;
mod cellrect;
use cellrect::CellRect;
use snake::{Snake, MovingDir};
use enemy::Enemy;

use std::time::{SystemTime, Duration};

use rand::Rng;

pub const cellsize: u8 = 25;
pub const cellcols: u8 = 30;
pub const cellrows: u8 = 23;

struct MainState {
	snake: Snake,
	enemies: Vec<Enemy>,
	food: Point2<i32>,
	next_update_time: i32,
	update_delay: i32,
	starttime: SystemTime,
	last_update_time: SystemTime,
	last_counted_second_time: SystemTime,
	fps_counter: u16,
	last_fps: u16,
	time_to_update: i64,
	font: graphics::Font,
	drawcount: u32,
	update_count: u32,
	scores: Vec<u16>,
	showscores: bool,
	active_food_mesh: u8,
	food_meshes: Vec<graphics::Mesh>,
	grass: graphics::Mesh,
	enemy_spawn_rate: f32,
}

impl MainState {
	fn new(ctx: &mut Context) -> GameResult<MainState> {
		let font = graphics::Font::new(ctx, "/fonts/kenvector_future_thin2.ttf")?;
		let grass = create_mesh::create_mesh(ctx, "/images/grass.png", cellcols as u16*cellsize as u16, cellcols as u16 *cellsize as u16);
		let food = MainState::random_position();
		let food_meshes = read_dir("e:/Dima/Projects/rust/rust_snake/target/debug/resources/images/food")?.map(|f| create_mesh::create_mesh(ctx, format!("/images/food/{}", f.unwrap().path().file_name().unwrap().to_str().unwrap()).as_str(), cellsize.into(), cellsize.into())).collect();
		let s = MainState {snake: Snake::new(ctx), food, next_update_time: 0, update_delay: 1000, font, starttime: SystemTime::now(), drawcount: 0, last_update_time: SystemTime::now(), time_to_update: 0, scores: vec![], showscores: false, food_meshes, active_food_mesh: 0, grass, last_counted_second_time: SystemTime::now(), fps_counter: 0, last_fps: 0, enemies: vec![Enemy::new(ctx)], enemy_spawn_rate: 0.03, update_count: 0};
		Ok(s)
	}
}

use std::rc::Rc;

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.snake.hande_input(keyboard::pressed_keys(ctx));

		let dt = self.last_update_time.elapsed().unwrap().as_micros();
		self.time_to_update -= dt as i64;
		self.last_update_time = SystemTime::now();

		self.next_update_time -= 1;
		if self.time_to_update < 0 {
			self.next_update_time += self.update_delay;
			self.snake.update();
			let occupied_cells = Vec::with_capacity(200);
			let occupied_cells = self.occupied_cells(occupied_cells);
			let mut lose = false;
			let mut enemy_ate_food = false;
			for e in self.enemies.iter_mut() {
				e.update(occupied_cells.as_slice());
				if e.overlapping(&CellRect {pos: self.food, w: 1, h: 1}) {
					enemy_ate_food = true;
				}
			}
			if enemy_ate_food {
				self.respawn_food();
			}
			if self.snake.eating_itself() || self.snake.out_of_bounds(0,0,(cellcols - 1) as i32,(cellrows - 1) as i32) {
				lose = true;
			}
			for e in &self.enemies {
				if e.overlapping(&CellRect {pos: self.snake.parts[0], w: 1, h: 1}) {
					lose = true;
				}
			}
			if lose {
				self.lose(ctx);
			}
			if self.snake.colliding_with_food(&self.food) {
				self.snake.grow();
				while self.snake.overlapping(&CellRect {pos: self.food, w: 1, h: 1}) {
					self.respawn_food();
				}
			}
			if rand::thread_rng().gen_range(0.0, 100.0) < self.enemy_spawn_rate && self.enemies.len() < 10 {
				self.spawn_enemy(ctx);
			}
			self.time_to_update += self.update_delay as i64;
			self.update_count += 1;
		}


		Ok(())
	}

	

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::draw(ctx, &self.grass, (na::Point2::<f32>::new((cellsize as f32 * cellcols as f32)/2.0,(cellsize as f32 * cellrows as f32)/2.0),));

		self.snake.draw(ctx);
		for e in &self.enemies {
			e.draw(ctx);
		}

		graphics::draw(ctx, &self.food_meshes[self.active_food_mesh as usize], (na::Point2::<f32>::new(self.food.x as f32 * cellsize as f32 + cellsize as f32/2.0, self.food.y as f32 * cellsize as f32 + cellsize as f32/2.0),));

		let from_last_counted_second_time = self.last_counted_second_time.elapsed().unwrap().as_micros();

		if from_last_counted_second_time > 1000000 {
			self.last_counted_second_time = SystemTime::now();
			self.last_fps = self.fps_counter;
			self.fps_counter = 0;
		} else {
			self.fps_counter += 1;
		}

		let text = graphics::Text::new((format!("FPS: {}", self.last_fps), self.font, 18.0));
		graphics::draw(ctx, &text, (na::Point2::<f32>::new(10.0, 10.0),graphics::Color::new(0.05,0.05,0.05,1.0)));
		let text = graphics::Text::new((format!("Score: {}", self.snake.parts.len()), self.font, 18.0));
		graphics::draw(ctx, &text, (na::Point2::<f32>::new(600.0, 10.0),graphics::Color::new(0.05,0.05,0.05,1.0)));

		if self.showscores {
			let text = graphics::Text::new((format!("High scores: "), self.font, 18.0));
			graphics::draw(ctx, &text, (na::Point2::<f32>::new(600.0, 30.0),graphics::Color::new(0.05,0.05,0.05,1.0)));
			let mut scores_y = 50.0;
			for s in &self.scores {
				let text = graphics::Text::new((format!("{}", s), self.font, 18.0));
				graphics::draw(ctx, &text, (na::Point2::<f32>::new(600.0, scores_y),graphics::Color::new(0.05,0.05,0.05,1.0)));
				scores_y += 22.0;
			}
		}

		graphics::present(ctx)?;
		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
		match key {
			KeyCode::Tab => self.showscores = !self.showscores,
			_ => (),
		};
	}
}

impl MainState {
	pub fn spawn_enemy(&mut self, ctx: &mut Context) {
		let mut en = Enemy::new(ctx);
		en.rect.pos = self.random_edge_valid_rect_spawn_position(en.rect.w, en.rect.h);
		self.enemies.push(en);
	}

	fn occupied_cells(&mut self, mut cellsvec: Vec<CellRect>) -> Vec<CellRect> {
		for oe in &self.enemies {
			cellsvec.push(oe.rect.clone());
		}
		cellsvec = self.snake.occupied_cells(cellsvec);
		cellsvec
	}

	pub fn random_position() -> Point2<i32> {
		Point2::from_slice(&[rand::thread_rng().gen_range(0, cellcols as i32),rand::thread_rng().gen_range(0, cellrows as i32)])
	}

	pub fn respawn_food(&mut self) {
		self.food = Self::random_position();
		self.active_food_mesh = rand::thread_rng().gen_range(0, self.food_meshes.len() as u8);
	}

	fn lose(&mut self, ctx: &mut Context) {
		let newscore = self.snake.parts.len() as u16;
		if !self.scores.contains(&newscore) {
			self.scores.push(newscore);
			self.scores.sort();
			self.scores.reverse();
		}
		self.snake = Snake::new(ctx);
		self.enemies.clear();
		self.respawn_food();
	}

	fn random_edge_valid_rect_spawn_position(&self, rectwidth: i32, rectheight: i32) -> Point2<i32> {
		'outer: loop {
			let random_p = match MovingDir::random() {
				MovingDir::WEST => Point2::<i32>::from_slice(&[0, rand::thread_rng().gen_range(0, cellrows as i32 + 1 - rectwidth)]),
				MovingDir::EAST => Point2::<i32>::from_slice(&[(cellcols as i32 - rectwidth).into(), rand::thread_rng().gen_range(0, cellrows as i32 + 1 - rectwidth)]),
				MovingDir::NORTH => Point2::<i32>::from_slice(&[rand::thread_rng().gen_range(0, cellcols as i32 + 1 - rectheight), 0]),
				MovingDir::SOUTH => Point2::<i32>::from_slice(&[rand::thread_rng().gen_range(0, cellcols as i32 + 1 - rectwidth), cellrows as i32+1 - rectheight]),
			};

			let rect = CellRect {
				pos: random_p,
				w: rectwidth,
				h: rectheight,
			};

			for en in &self.enemies {
				if en.rect.overlapping(&rect) {
					continue 'outer;
				}
			}
			if self.snake.overlapping(&rect) {
				continue 'outer;
			}
			if random_p == self.food {
				continue 'outer;
			}

			break random_p
		}
	}
}

pub fn main() -> GameResult {
	let window = ggez::conf::WindowSetup {
		title: "Snake".to_owned(),
		samples: ggez::conf::NumSamples::Zero,
		vsync: false,
		icon: "".to_owned(),
		srgb: true,
	};
	let windowmode = ggez::conf::WindowMode {
		width: cellcols as f32*cellsize as f32,
		height: cellrows as f32*cellsize as f32,
		maximized: false,
		fullscreen_type: ggez::conf::FullscreenType::Windowed,
		borderless: false,
		min_width: 0.0,
		max_width: 0.0,
		min_height: 0.0,
		max_height: 0.0,
		resizable: false,
	};
	let conf = ggez::conf::Conf {
		window_mode: windowmode,
		window_setup: window,
		backend: ggez::conf::Backend::default(),
		modules: ggez::conf::ModuleConf::default(),
	};
	// let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let cb = ggez::ContextBuilder::new("super_simple", "ggez").conf(conf);
	let (ref mut ctx, event_loop) = &mut cb.build()?;
	let state = &mut MainState::new(ctx)?;
	event::run(ctx, event_loop, state)
}