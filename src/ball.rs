use macroquad::prelude::*;
use crate::brick::Brick;
use crate::collider::Collider;
use crate::entity::Entity;
use crate::paddle::Paddle;
use crate::sound::SoundSystem;

pub const BALL_RADIUS: f32 = 8.0;

pub struct Ball {
	radius: f32,
	dx: f32,
	dy: f32,
	pub is_moving: bool,
	x: f32,
	y: f32
}

impl Ball {
	pub fn new() -> Self {
		const ANGLE: f32 = -45.0_f32.to_radians();

		Ball {
			radius: BALL_RADIUS,
			dx: ANGLE.sin() * 4.0,
			dy: ANGLE.cos() * 4.0,
			is_moving: false,
			x: screen_width() / 2.0,
			y: screen_height() / 1.55,
		}
	}

	pub fn update(
		&mut self,
		paddle: &Paddle,
		bricks: &mut Vec<Brick>,
		score: &mut usize,
		sound_system: &mut SoundSystem
	) {
		if !self.is_moving {
			self.x += paddle.speed;
			return;
		}

		self.x += self.dx;
		self.y -= self.dy;

		let collider: Collider = self.collider();
		let paddle_collider: Collider = paddle.collider();

		if collider.is_colliding(&paddle_collider) {
			self.y += paddle_collider.top() - collider.bottom();

			self.flip_vertical(sound_system);
			if paddle.speed.abs() > 0.0 && (
				(paddle.speed.is_sign_positive() && self.dx.is_sign_negative()) ||
				(paddle.speed.is_sign_negative() && self.dx.is_sign_positive())
			) {
				self.flip_horizontal(sound_system);
			}
		}

		bricks.retain(|brick| {
			let delete: bool = {
				if collider.is_colliding(&brick.collider()) {
					self.flip_vertical(sound_system);
					*score += 100;
					sound_system.queue_sound(55);
					true
				} else { false }
			};
			!delete
		});

		let left_boundary: f32 = self.radius;
		let right_boundary: f32 = screen_width() - self.radius;
		let top_boundary: f32 = self.radius;
		let bottom_boundary: f32 = screen_height() - self.radius;

		if self.x < left_boundary {
			self.x = left_boundary;
			self.flip_horizontal(sound_system);
		} else if self.x > right_boundary {
			self.x = right_boundary;
			self.flip_horizontal(sound_system);
		}

		if self.y < top_boundary {
			self.y = top_boundary;
			self.flip_vertical(sound_system);
		} else if self.y > bottom_boundary {
			self.y = bottom_boundary;
			self.flip_vertical(sound_system);
			panic!("you lost")
		}
	}

	fn flip_horizontal(&mut self, sound_system: &mut SoundSystem) {
		sound_system.queue_sound(48);
		self.dx = -self.dx;
	}

	fn flip_vertical(&mut self, sound_system: &mut SoundSystem) {
		sound_system.queue_sound(52);
		self.dy = -self.dy;
	}
}

impl Entity for Ball {
	fn collider(&self) -> Collider {
		Collider::new(self.x, self.y, self.radius, self.radius)
	}

	fn draw(&self) {
		draw_circle(self.x, self.y, self.radius, WHITE);
	}
}
