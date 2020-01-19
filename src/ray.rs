use crate::{
	intersect::{Hit, Intersect},
	products::{CrossProduct, DotProduct},
	triangle::Triangle,
	vec3::Vec3,
};

#[derive(Debug)]
pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3 {
		self.origin + t * self.direction
	}
}

impl Intersect<Triangle<Vec3>> for Ray {
	type Record = Hit;
	/// An implementation of the Möller-Trumbore Ray-Triangle Intersection
	/// algorithm.
	///
	/// This algorithm uses a fair amount of computation to perform its task of
	/// finding the intersection point.
	fn intersect(&self, tri: Triangle<Vec3>) -> Option<Hit> {
		let ab: Vec3 = tri.1 - tri.0;
		let ac: Vec3 = tri.2 - tri.0;

		let norm: Vec3 = self.direction.cross(ac);
		let angle = ab.dot(norm);

		if angle > -core::f32::EPSILON && angle < core::f32::EPSILON {
			return None;
		}

		let f: f32 = 1.0 / angle;
		let offset: Vec3 = self.origin - tri.0;

		let u = f * offset.dot(norm);

		if u < 0.0 || u > 1.0 {
			return None;
		}

		let qvec: Vec3 = offset.cross(ab);

		let v = f * self.direction.dot(qvec);

		if v < 0.0 || u + v > 1.0 {
			return None;
		}

		let t = f * ac.dot(qvec);

		Some(Hit {
			time: t,
			point: self.at(t),
		})
	}
}

#[test]
fn intersect_correct() {
	let ray: Ray = Ray {
		origin: Vec3(0_f32, 0_f32, 0_f32),
		direction: Vec3(1_f32, 0_f32, 0_f32),
	};

	let triangle: Triangle<Vec3> = Triangle(
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
