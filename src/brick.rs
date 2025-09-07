use macroquad::prelude::*;
use crate::collider::Collider;
use crate::entity::Entity;

pub const BRICK_WIDTH: f32 = 32.0;
pub const BRICK_HEIGHT: f32 = 8.0;

pub struct Brick {
	width: f32,
	height: f32,
	x: f32,
	y: f32
}

impl Brick {
	pub fn new(x: f32, y: f32) -> Self {
		Brick {
			width: BRICK_WIDTH,
			height: BRICK_HEIGHT,
			x,
			y
		}
	}
}

impl Entity for Brick {
	fn collider(&self) -> Collider { Collider::new(self.x, self.y, self.width, self.height) }

	fn draw(&self) {
		draw_rectangle(self.x, self.y, self.width, self.height, WHITE)
	}
}
