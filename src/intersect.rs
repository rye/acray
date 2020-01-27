use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
	pub time: f64,
	pub point: Vec3,
	pub unit_normal: Vec3,
}

pub trait Intersect<T> {
	type Record;
	fn intersect(&self, other: T) -> Option<Self::Record>;
}

use core::cmp::Ordering;

impl PartialOrd for Hit {
	fn partial_cmp(&self, other: &Hit) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Eq for Hit {}

impl Ord for Hit {
	fn cmp(&self, other: &Self) -> Ordering {
		self
			.time
			.partial_cmp(&other.time)
			.unwrap_or(Ordering::Equal)
	}
}
