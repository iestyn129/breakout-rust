mod ball;
mod paddle;
mod entity;
mod debug;
mod collider;
mod world;
mod brick;
mod sound;

use std::thread::sleep;
use std::time::Duration;
use macroquad::audio::Sound;
use macroquad::prelude::*;
use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::sound::generate_square_wave;
use crate::world::World;

fn window() -> Conf { Conf {
	window_title: "breakout".to_owned(),
	window_width: 450,
	window_height: 600,
	window_resizable: false,
	fullscreen: false,
	..Default::default()
} }

#[macroquad::main(window)]
async fn main() {
	const FPS: f32 = 1.0 / 60.0;
	const SOUND_DURATION: f32 = 0.1;
	const SOUND_SAMPLE_RATE: u32 = 176_400;

	let mut world: World = World::new(
		Paddle::new(),
		Ball::new()
	);

	for note in 48..=72 {
		let freq: f32 = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
		let sound: Sound = generate_square_wave(freq, SOUND_DURATION, SOUND_SAMPLE_RATE).await;
		world.sound_system.register_sound(note as usize, sound);
	}

	loop {
		if is_key_pressed(KeyCode::Space) && !world.started {
			world.start();
		}

		world.update();

		clear_background(BLACK);
		world.draw();

		next_frame().await;

		let frame_time: f32 = get_frame_time();
		if frame_time < FPS {
			let sleep_time: f32 = FPS - frame_time;
			sleep(Duration::from_secs_f32(sleep_time));
		}
	}
}
