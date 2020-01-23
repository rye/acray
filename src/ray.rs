use core::borrow::Borrow;

use crate::{
	intersect::{Hit, Intersect},
	products::{CrossProduct, DotProduct},
	sphere::Sphere,
	triangle::Triangle,
	vec3::Vec3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
	pub(crate) origin: Vec3,
	pub(crate) direction: Vec3,
	pub(crate) t_offset: f32,
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3 {
		self.origin + (t - self.t_offset) * self.direction
	}

	pub fn new(origin: Vec3, direction: Vec3) -> Self {
		Self {
			origin,
			direction,
			t_offset: 0.0_f32,
		}
	}
}

impl<T> Intersect<T> for Ray
where
	T: Borrow<Triangle<Vec3>>
{
	type Record = Hit;

	/// An implementation of the Möller-Trumbore Ray-Triangle Intersection
	/// algorithm.
	///
	/// This algorithm uses a fair amount of computation to perform its task of
	/// finding the intersection point.
	fn intersect(&self, tri: T) -> Option<Hit> {
		let tri: &Triangle<Vec3> = tri.borrow();
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

		let normal: Vec3 = ac.cross(ab);

		let unit_normal: Option<Vec3> = Some(normal / normal.mag());

		Some(Hit {
			time: t,
			point: self.at(t),
			unit_normal,
		})
	}
}

impl Intersect<Sphere> for Ray {
	type Record = Vec<Hit>;
	fn intersect(&self, sphere: Sphere) -> Option<Self::Record> {
		use core::f32::EPSILON;

		let oc: Vec3 = sphere.origin - self.origin;
		let tca: f32 = oc.dot(self.direction);
		let d2: f32 = oc.dot(oc) - tca * tca;
		let radius2: f32 = sphere.radius.powi(2);

		if d2 > radius2 {
			return None;
		}

		let thc: f32 = (radius2 - d2).sqrt();

		let record: Vec<Hit> = match thc {
			thc if (-EPSILON..=EPSILON).contains(&thc) => {
				let t: f32 = tca;

				let p: Vec3 = self.at(t);

				let n: Vec3 = p - sphere.origin;

				vec![Hit {
					time: t,
					point: self.at(t),
					unit_normal: Some(n / n.mag()),
				}]
			}
			_ => {
				let t0: f32 = tca - thc;
				let t1: f32 = tca + thc;

				let p0: Vec3 = self.at(t0);
				let p1: Vec3 = self.at(t1);

				let n0: Vec3 = p0 - sphere.origin;
				let n1: Vec3 = p1 - sphere.origin;

				vec![
					Hit {
						time: t0,
						point: p0,
						unit_normal: Some(n0 / n0.mag()),
					},
					Hit {
						time: t1,
						point: p1,
						unit_normal: Some(-(n1 / n1.mag())),
					},
				]
			}
		};

		Some(record)
	}
}

#[cfg(test)]
mod tests;
