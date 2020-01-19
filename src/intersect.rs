use crate::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
	pub time: f32,
	pub point: Vec3,
}

pub trait Intersect<Intersectable> {
	type Record;
	fn intersect(&self, other: Intersectable) -> Option<Self::Record>;
}
