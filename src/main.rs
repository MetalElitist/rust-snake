use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context, GameResult};
use ggez::input::keyboard;

mod snake;
use crate::snake::{Snake};

use std::time::{SystemTime, Duration};

use rand::Rng;

pub const cellsize: u8 = 25;
pub const cellcols: u8 = 30;
pub const cellrows: u8 = 23;

struct MainState {
	snake: Snake,
	food: Point2<i32>,
	next_update_time: i32,
	update_delay: i32,
	starttime: SystemTime,
	last_update_time: SystemTime,
	time_to_update: i64,
	font: graphics::Font,
	drawcount: u32,
}

impl MainState {
	fn new(ctx: &mut Context) -> GameResult<MainState> {

		let food = MainState::new_food_pos();
		let font = graphics::Font::new(ctx, "/nyala.ttf")?;
		let s = MainState {snake: Snake::new(ctx), food: food, next_update_time: 0, update_delay: 30, font, starttime: SystemTime::now(), drawcount: 0, last_update_time: SystemTime::now(), time_to_update: 0};
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
			if self.snake.eating_itself() || self.snake.out_of_bounds(0,0,(cellcols - 1) as i32,(cellrows - 1) as i32) {
				self.lose(ctx);
			}
			if self.snake.colliding_with_food(&self.food) {
				self.snake.grow();
				while self.snake.overlapping(self.food) {
					self.food = MainState::new_food_pos();
				}
			}
			self.time_to_update += 70000;
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, [0.9, 0.9, 0.9, 1.0].into());

		self.snake.draw(ctx);

		let food = graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(),
				graphics::Rect::new(0.0, 0.0, cellsize as f32, cellsize as f32), 
				graphics::Color::new(0.05, 0.9, 0.05, 1.0),
			).unwrap();

		graphics::draw(ctx, &food, (na::Point2::<f32>::new(self.food.x as f32 * cellsize as f32, self.food.y as f32 * cellsize as f32),));


		let gametime = self.starttime.elapsed().unwrap().as_millis();

		let text = graphics::Text::new((format!("FPS: {}", (self.drawcount as f64 / (gametime as f64 / 1000.0)) as u32), self.font, 18.0));
		graphics::draw(ctx, &text, (na::Point2::<f32>::new(10.0, 10.0),graphics::Color::new(0.05,0.05,0.05,1.0)));
		let text = graphics::Text::new((format!("Score: {}", self.snake.parts.len()), self.font, 18.0));
		graphics::draw(ctx, &text, (na::Point2::<f32>::new(600.0, 10.0),graphics::Color::new(0.05,0.05,0.05,1.0)));

		graphics::present(ctx)?;
		self.drawcount += 1;
		Ok(())
	}

}

impl MainState {
	pub fn new_food_pos() -> Point2<i32> {
		Point2::from_slice(&[rand::thread_rng().gen_range(0, cellcols as i32),rand::thread_rng().gen_range(0, cellrows as i32)])
	}

	fn lose(&mut self, ctx: &mut Context) {
		self.snake = Snake::new(ctx);
		self.food = MainState::new_food_pos();
	}
}

pub fn main() -> GameResult {
	let window = ggez::conf::WindowSetup {
		title: "An easy, good game".to_owned(),
		samples: ggez::conf::NumSamples::Zero,
		vsync: false,
		icon: "".to_owned(),
		srgb: true,
	};
	let windowmode = ggez::conf::WindowMode {
		width: 30.0*cellsize as f32,
		height: 23.0*cellsize as f32,
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