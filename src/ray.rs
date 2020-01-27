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
		self.origin + t * self.direction
	}

	pub fn new(origin: Vec3, direction: Vec3) -> Self {
		Self {
			origin,
			direction,
			t_offset: 0.0_f32,
		}
	}
}

impl Intersect<&Triangle<Vec3>> for Ray {
	type Record = Hit;

	/// An implementation of the Möller-Trumbore Ray-Triangle Intersection
	/// algorithm.
	///
	/// This algorithm uses a fair amount of computation to perform its task of
	/// finding the intersection point.
	fn intersect(&self, tri: &Triangle<Vec3>) -> Option<Hit> {
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

		let unit_normal: Vec3 = ac.cross(ab).unit();

		Some(Hit {
			time: t + self.t_offset,
			point: self.at(t),
			unit_normal,
		})
	}
}

impl Intersect<&Sphere> for Ray {
	type Record = Vec<Hit>;
	fn intersect(&self, sphere: &Sphere) -> Option<Self::Record> {
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
				let time: f32 = tca + self.t_offset;

				let point: Vec3 = self.at(tca);

				let unit_normal: Vec3 = (point - sphere.origin).unit();

				vec![Hit {
					time,
					point,
					unit_normal,
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
						time: t0 + self.t_offset,
						point: p0,
						unit_normal: n0 / n0.mag(),
					},
					Hit {
						time: t1 + self.t_offset,
						point: p1,
						unit_normal: -(n1 / n1.mag()),
					},
				]
			}
		};

		Some(record)
	}
}

#[cfg(test)]
mod tests;
