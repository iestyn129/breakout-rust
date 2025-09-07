pub struct Collider {
	pub x: f32,
	pub y: f32,
	width: f32,
	height: f32,
}

impl Collider {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Collider { x, y, width, height }
	}

	pub fn is_colliding(&self, other: &Collider) -> bool {
		if self.right() < other.left() || self.left() > other.right() {
			return false;
		}

		if self.bottom() < other.top() || self.top() > other.bottom() {
			return false;
		}

		true
	}

	pub fn left(&self) -> f32 { self.x }

	pub fn right(&self) -> f32 { self.x + self.width }

	pub fn top(&self) -> f32 { self.y }

	pub fn bottom(&self) -> f32 { self.y + self.height }
}
