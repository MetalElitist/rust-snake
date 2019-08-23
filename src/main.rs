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
use snake::{Snake};
use enemy::Enemy;

use std::time::{SystemTime, Duration};

use rand::Rng;

pub const cellsize: u8 = 25;
pub const cellcols: u8 = 30;
pub const cellrows: u8 = 23;

struct MainState {
	snake: Snake,
	enemy: Enemy,
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
	scores: Vec<u16>,
	showscores: bool,
	active_food_mesh: u8,
	food_meshes: Vec<graphics::Mesh>,
	grass: graphics::Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> GameResult<MainState> {
		let font = graphics::Font::new(ctx, "/nyala.ttf")?;
		let grass = create_mesh::create_mesh(ctx, "/images/grass.png", cellcols as u16*cellsize as u16, cellcols as u16 *cellsize as u16);
		let food = MainState::random_position();
		let food_meshes = read_dir("e:/Dima/Projects/rust/rust_snake/target/debug/resources/images/food")?.map(|f| create_mesh::create_mesh(ctx, format!("/images/food/{}", f.unwrap().path().file_name().unwrap().to_str().unwrap()).as_str(), cellsize.into(), cellsize.into())).collect();
		let s = MainState {snake: Snake::new(ctx), food, next_update_time: 0, update_delay: 70000, font, starttime: SystemTime::now(), drawcount: 0, last_update_time: SystemTime::now(), time_to_update: 0, scores: vec![], showscores: false, food_meshes, active_food_mesh: 0, grass, last_counted_second_time: SystemTime::now(), fps_counter: 0, last_fps: 0, enemy: Enemy::new(ctx)};
		Ok(s)
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.snake.hande_input(keyboard::pressed_keys(ctx));

		let dt = self.last_update_time.elapsed().unwrap().as_micros();
		self.time_to_update -= dt as i64;
		self.last_update_time = SystemTime::now();

		self.next_update_time -= 1;
		if self.time_to_update < 0 {
			self.next_update_time = self.update_delay;
			self.snake.update();
			self.enemy.update();
			if self.snake.eating_itself() || self.snake.out_of_bounds(0,0,(cellcols - 1) as i32,(cellrows - 1) as i32) {
				self.lose(ctx);
			}
			if self.snake.colliding_with_food(&self.food) {
				self.snake.grow();
				while self.snake.overlapping(self.food) {
					self.respawn_food();
				}
			}
			self.time_to_update += self.update_delay as i64;
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::draw(ctx, &self.grass, (na::Point2::<f32>::new((cellsize as f32 * cellcols as f32)/2.0,(cellsize as f32 * cellrows as f32)/2.0),));

		self.snake.draw(ctx);
		self.enemy.draw(ctx);

		graphics::draw(ctx, &self.food_meshes[self.active_food_mesh as usize], (na::Point2::<f32>::new(self.food.x as f32 * cellsize as f32 + cellsize as f32/2.0, self.food.y as f32 * cellsize as f32 + cellsize as f32/2.0),));

		let from_last_counted_second_time = self.last_counted_second_time.elapsed().unwrap().as_micros();

		if from_last_counted_second_time > 1000000 {
			self.last_counted_second_time = SystemTime::now();
			self.last_fps = self.fps_counter;
			self.fps_counter = 0;
		} else {
			self.fps_counter += 1;
		}

		let gametime = self.starttime.elapsed().unwrap().as_millis();

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
		self.drawcount += 1;
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
		}
		self.snake = Snake::new(ctx);
		self.respawn_food();
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