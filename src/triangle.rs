use crate::{vec3::Vec3};

#[derive(Debug, PartialEq)]
pub struct Triangle<V>(pub V, pub V, pub V)
where
	V: Sized + Copy + Clone + core::fmt::Debug;
