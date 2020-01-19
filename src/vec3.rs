use core::borrow::Borrow;
use core::ops::{Add, Mul, Sub};

use crate::products::{CrossProduct, DotProduct};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl DotProduct for Vec3 {
	type Output = f32;
	fn dot<T: Borrow<Self>>(&self, other: T) -> Self::Output {
		let other: &Self = other.borrow();
		(self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
	}
}

impl CrossProduct for Vec3 {
	fn cross<T: Borrow<Self>>(&self, other: T) -> Self {
		let other: &Self = other.borrow();
		Self(
			self.1 * other.2 - self.2 * other.1,
			self.2 * other.0 - self.0 * other.2,
			self.0 * other.1 - self.1 * other.0,
		)
	}
}

impl Vec3 {
	pub fn mag(&self) -> f32 {
		(self.0.powi(2) as f32 + self.1.powi(2) as f32 + self.2.powi(2) as f32).sqrt()
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 - vec.0, self.1 - vec.1, self.2 - vec.2)
	}
}

impl Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, vec: Vec3) -> Self::Output {
		Vec3(self * vec.0, self * vec.1, self * vec.2)
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, scalar: f32) -> Self::Output {
		Vec3(scalar * self.0, scalar * self.1, scalar * self.2)
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 + vec.0, self.1 + vec.1, self.2 + vec.2)
	}
}
