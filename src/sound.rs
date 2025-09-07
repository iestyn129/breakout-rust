use hound::{WavSpec, WavWriter, SampleFormat};
use macroquad::audio::*;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub struct SoundSystem {
	sound_map: HashMap<usize, Sound>,
	sound_queue: Vec<usize>,
	sound_cooldowns: HashMap<usize, usize>
}

impl SoundSystem {
	pub fn new() -> Self {
		SoundSystem {
			sound_map: HashMap::new(),
			sound_queue: vec![],
			sound_cooldowns: HashMap::new()
		}
	}

	pub fn register_sound(&mut self, note: usize, sound: Sound) {
		self.sound_map.insert(note, sound);
		self.sound_cooldowns.insert(note, 0);
	}

	pub fn queue_sound(&mut self, note: usize) {
		self.sound_queue.push(note);
	}

	pub fn update(&mut self) {
		for note in self.sound_queue.drain(0..) {
			if let Some(cooldown) = self.sound_cooldowns.get_mut(&note) {
				if *cooldown <= 0 {
					*cooldown = 6;
					if let Some(sound) = self.sound_map.get(&note) {
						play_sound_once(sound);
					}
				}
			}
		}

		for cooldown in self.sound_cooldowns.values_mut() {
			if *cooldown > 0 {
				*cooldown -= 1;
			}
		}
	}
}

pub async fn generate_square_wave(frequency: f32, duration: f32, sample_rate: u32) -> Sound {
	let samples: usize = (duration * sample_rate as f32) as usize;
	let mut buffer: Vec<f32> = Vec::with_capacity(samples);
	let period: f32 = sample_rate as f32 / frequency;

	for i in 0..samples {
		let value: f32 = if (i as f32 % period) < (period / 2.0)
			{ 0.5 } else { -0.5 };
		buffer.push(value);
	}

	let pcm: Vec<i16> = buffer.iter().map(|&s| (s * i16::MAX as f32) as i16).collect();
	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
	let mut writer: WavWriter<&mut Cursor<Vec<u8>>> = WavWriter::new(&mut cursor, WavSpec {
		channels: 1,
		sample_rate,
		bits_per_sample: 16,
		sample_format: SampleFormat::Int,
	}).unwrap();
	for sample in &pcm {
		writer.write_sample(*sample).unwrap();
	}
	writer.finalize().unwrap();

	load_sound_from_bytes(&cursor.into_inner()).await.unwrap()
}
