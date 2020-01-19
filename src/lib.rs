use core::{
	borrow::Borrow,
	ops::{Add, Mul, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
	pub fn mag(&self) -> f32 {
		(self.0.powi(2) as f32 + self.1.powi(2) as f32 + self.2.powi(2) as f32).sqrt()
	}

	pub fn dot<T: Borrow<Self>>(&self, other: T) -> f32 {
		let other: &Self = other.borrow();
		(self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
	}

	pub fn cross<T: Borrow<Self>>(&self, other: T) -> Self {
		let other: &Self = other.borrow();
		Self(
			self.1 * other.2 - self.2 * other.1,
			self.2 * other.0 - self.0 * other.2,
			self.0 * other.1 - self.1 * other.0,
		)
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

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 + vec.0, self.1 + vec.1, self.2 + vec.2)
	}
}

#[derive(Debug)]
pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3 {
		self.origin + t * self.direction
	}

	/// An implementation of the Möller-Trumbore Ray-Triangle Intersection
	/// algorithm.
	///
	/// This algorithm uses a fair amount of computation to perform its task of
	/// finding the intersection point.
	pub fn intersect(&self, tri: Triangle) -> Option<Hit> {
		let ab: Vec3 = tri.1 - tri.0;
		let ac: Vec3 = tri.2 - tri.0;

		let h: Vec3 = self.direction.cross(ac);
		let a = ab.dot(h);

		if a > -core::f32::EPSILON && a < core::f32::EPSILON {
			return None;
		}

		let f: f32 = 1.0 / a;
		let s = self.origin - tri.0;
		let u = f * s.dot(h);

		if u < 0.0 || u > 1.0 {
			return None;
		}

		let q = s.cross(ab);
		let v = f * self.direction.dot(q);

		if v < 0.0 || u + v > 1.0 {
			return None;
		}

		let t = f * ac.dot(q);

		Some(Hit {
			time: t,
			point: self.at(t),
		})
	}
}

pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
	time: f32,
	point: Vec3,
}

#[test]
fn intersect_correct() {
	let ray: Ray = Ray {
		origin: Vec3(0_f32, 0_f32, 0_f32),
		direction: Vec3(1_f32, 0_f32, 0_f32),
	};

	let triangle: Triangle = Triangle(
		Vec3(2_f32, 1_f32, 0_f32),
		Vec3(2_f32, -1_f32, 1_f32),
		Vec3(2_f32, -1_f32, -1_f32),
	);

	let expected_hit_record: Hit = Hit {
		time: 2_f32,
		point: Vec3(2_f32, 0_f32, 0_f32),
	};

	assert_eq!(ray.intersect(triangle), Some(expected_hit_record));
}
