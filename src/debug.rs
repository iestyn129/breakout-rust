use macroquad::prelude::*;
use crate::collider::Collider;
use crate::entity::Entity;

const LINE_THICKNESS: f32 = 2.0;

pub struct Debug;

impl Debug {
	pub fn new() -> Self { Debug {} }
}

impl Entity for Debug {
	fn collider(&self) -> Collider {
		Collider::new(0.0, 0.0, 0.0, 0.0)
	}

	fn draw(&self) {
		let vertical_x: f32 = screen_width() / 2.0;
		let horizontal_y: f32 = screen_height() / 2.0;
		draw_line(vertical_x, 0.0, vertical_x, screen_height(), LINE_THICKNESS, GRAY);
		draw_line(0.0, horizontal_y, screen_width(), horizontal_y, LINE_THICKNESS, GRAY);
	}
}
