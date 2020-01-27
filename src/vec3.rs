use core::borrow::Borrow;
use core::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

use crate::products::{CrossProduct, DotProduct};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl DotProduct for Vec3 {
	type Output = f64;
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
	pub fn mag(&self) -> f64 {
		(self.0.powi(2) as f64 + self.1.powi(2) as f64 + self.2.powi(2) as f64).sqrt()
	}

	pub fn from_components_with_mag(components: (f64, f64, f64), magnitude: f64) -> Self {
		let unit: Vec3 = Vec3(components.0, components.1, components.2).unit();
		unit * magnitude
	}

	pub fn unit(self) -> Self {
		self / self.mag()
	}
}

impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3(-self.0, -self.1, -self.2)
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 + vec.0, self.1 + vec.1, self.2 + vec.2)
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, scalar: f64) -> Self::Output {
		Vec3(self.0 / scalar, self.1 / scalar, self.2 / scalar)
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 - vec.0, self.1 - vec.1, self.2 - vec.2)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, vec: Vec3) -> Self::Output {
		Vec3(self * vec.0, self * vec.1, self * vec.2)
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, scalar: f64) -> Self::Output {
		Vec3(scalar * self.0, scalar * self.1, scalar * self.2)
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, scalar: f64) {
		self.0 /= scalar;
		self.1 /= scalar;
		self.2 /= scalar;
	}
}
