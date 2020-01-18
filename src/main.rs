//struct Vec2(f32, f32);
#[derive(Copy, Clone, Debug, PartialEq)]
struct Vec3(f32, f32, f32);
type Point = Vec3;

struct Ray {
	origin: Point,
	direction: Vec3,
}

struct Triangle(Point, Point, Point);

impl Ray {
	pub fn at(&self, t: f32) -> Vec3 {
		self.origin + t * self.direction
	}
}

impl Vec3 {
	pub fn mag(&self) -> f32 {
		(self.0.powi(2) as f32 + self.1.powi(2) as f32 + self.2.powi(2) as f32).sqrt()
	}

	pub fn dot<T: core::borrow::Borrow<Self>>(&self, other: T) -> f32 {
		let other: &Self = other.borrow();
		(self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
	}

	pub fn cross<T: core::borrow::Borrow<Self>>(&self, other: T) -> Self {
		let other: &Self = other.borrow();
		Self(
			self.1 * other.2 - self.2 * other.1,
			self.2 * other.0 - self.0 * other.2,
			self.0 * other.1 - self.1 * other.0,
		)
	}
}

impl core::ops::Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 - vec.0, self.1 - vec.1, self.2 - vec.2)
	}
}

impl core::ops::Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, vec: Vec3) -> Self::Output {
		Vec3(self * vec.0, self * vec.1, self * vec.2)
	}
}

impl core::ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, vec: Vec3) -> Self::Output {
		Vec3(self.0 + vec.0, self.1 + vec.1, self.2 + vec.2)
	}
}

/// An implementation of the Möller-Trumbore Ray-Triangle Intersection
/// algorithm.
///
/// This algorithm uses a fair amount of computation to perform its
/// task of finding the intersection point.
fn intersect(ray: Ray, tri: Triangle) -> Option<(f32, Vec3)> {
	let ab: Vec3 = tri.1 - tri.0;
	let ac: Vec3 = tri.2 - tri.0;

	let h: Vec3 = ray.direction.cross(ac);
	let a = ab.dot(h);

	if a > -core::f32::EPSILON && a < core::f32::EPSILON {
		return None;
	}

	let f: f32 = 1.0 / a;
	let s = ray.origin - tri.0;
	let u = f * s.dot(h);

	if u < 0.0 || u > 1.0 {
		return None;
	}

	let q = s.cross(ab);
	let v = f * ray.direction.dot(q);

	if v < 0.0 || u + v > 1.0 {
		return None;
	}

	let t = f * ac.dot(q);

	Some((t, ray.at(t)))
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

	assert_eq!(intersect(ray, triangle), Some((2_f32, Vec3(2_f32, 0_f32, 0_f32))));
}

fn main() {
	let ray = Ray {
		origin: Vec3(0.0_f32, 2.0_f32, 0.0_f32),
		direction: Vec3(-0.0_f32, -1.0_f32, 0.0_f32),
	};
	let tri = Triangle(
		Vec3(1.0_f32, 0.0_f32, 0.0_f32),
		Vec3(-1.0_f32, 0.0_f32, 1.0_f32),
		Vec3(-1.0_f32, 0.0_f32, -1.0_f32),
	);

	println!("{:?}", intersect(ray, tri));
}
