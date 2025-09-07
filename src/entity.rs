use crate::collider::Collider;

pub trait Entity {
	fn collider(&self) -> Collider;

	fn draw(&self);
}
