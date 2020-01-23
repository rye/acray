use crate::{vec3::Vec3};

pub struct Sphere {
	pub(crate) origin: Vec3,
	pub(crate) radius: f32,
}

impl Sphere {
	pub fn new(origin: Vec3, radius: f32) -> Self {
		Self { origin, radius }
	}
}
