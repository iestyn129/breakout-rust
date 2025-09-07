use macroquad::prelude::*;
use crate::ball::Ball;
use crate::brick::{Brick, BRICK_WIDTH, BRICK_HEIGHT};
use crate::debug::Debug;
use crate::entity::Entity;
use crate::paddle::Paddle;
use crate::sound::SoundSystem;

pub struct World {
	pub sound_system: SoundSystem,
	pub score: usize,
	pub paddle: Paddle,
	pub ball: Ball,
	pub bricks: Vec<Brick>,
	debug: Debug,
	pub started: bool
}

impl World {
	pub fn new(paddle: Paddle, ball: Ball) -> Self {
		const BRICKS_PER_ROW: f32 = 10.0;
		const BRICK_SPACING: f32 = 8.0;
		const BRICK_ROWS: usize = 14;

		let default_brick_x: f32 = (
			screen_width() - ((
				BRICK_WIDTH * BRICKS_PER_ROW
			) + (
				BRICK_SPACING * (BRICKS_PER_ROW - 1.0)
			))
		) / 2.0;

		let mut bricks: Vec<Brick> = vec![];
		let mut brick_x: f32 = default_brick_x;
		let mut brick_y: f32 = (BRICK_HEIGHT + BRICK_SPACING) * 2.0;

		for _ in 0..BRICK_ROWS {
			for _ in 0..BRICKS_PER_ROW as usize {
				bricks.push(Brick::new(brick_x, brick_y));
				brick_x += BRICK_WIDTH + BRICK_SPACING;
			}

			brick_x = default_brick_x;
			brick_y += BRICK_HEIGHT + BRICK_SPACING;
		}

		World {
			sound_system: SoundSystem::new(),
			score: 0,
			paddle,
			ball,
			bricks,
			debug: Debug::new(),
			started: false
		}
	}

	pub fn start(&mut self) {
		self.started = true;
		self.ball.is_moving = true;
	}

	pub fn stop(&mut self) {
		self.started = false;
		self.ball.is_moving = false;
	}

	pub fn update(&mut self) {
		self.paddle.update();
		self.ball.update(&self.paddle, &mut self.bricks, &mut self.score, &mut self.sound_system);
		self.sound_system.update();

		if self.bricks.len() <= 0 {
			self.stop();
		}
	}

	pub fn draw(&self) {
		self.debug.draw();
		self.paddle.draw();
		self.ball.draw();
		self.bricks.iter().for_each(|brick| brick.draw());

		draw_text_ex(format!("SCORE: {}", self.score).as_str(), 5.0, 20.0, TextParams {
			font_size: 30,
			color: WHITE,
			..Default::default()
		});
	}
}
