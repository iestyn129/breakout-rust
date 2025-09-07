use macroquad::prelude::*;
use crate::collider::Collider;
use crate::entity::Entity;

pub const PADDLE_WIDTH: f32 = 96.0;
pub const PADDLE_HEIGHT: f32 = 8.0;

pub struct Paddle {
	width: f32,
	height: f32,
	pub speed: f32,
	x: f32,
	y: f32
}

impl Paddle {
	pub fn new() -> Self {
		Paddle {
			width: PADDLE_WIDTH,
			height: PADDLE_HEIGHT,
			speed: 0.0,
			x: (screen_width() / 2.0) - (PADDLE_WIDTH / 2.0),
			y: (screen_height() * (4.0 / 5.0)) - (PADDLE_HEIGHT / 2.0),
		}
	}

	pub fn update(&mut self) {
		const STEP: f32 = 8.0;
		self.speed = 0.0;
		let mut x_mov: f32 = 0.0;
		let left_boundary: f32 = 0.0;
		let right_boundary: f32 = screen_width() - self.width;

		if is_key_down(KeyCode::Right) {
			x_mov += STEP;
		}
		if is_key_down(KeyCode::Left) {
			x_mov -= STEP;
		}

		self.x += x_mov;
		self.speed = x_mov;

		let old_x: f32 = self.x;
		if self.x <= left_boundary {
			self.x = left_boundary;
		} else if self.x >= right_boundary {
			self.x = right_boundary;
		}
		self.speed += self.x - old_x;
	}
}

impl Entity for Paddle {
	fn collider(&self) -> Collider {
		Collider::new(self.x, self.y, self.width, self.height)
	}

	fn draw(&self) {
		draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
	}
}
