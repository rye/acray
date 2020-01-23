use crate::{
	intersect::{Hit, Intersect, Intersectable},
	ray::Ray,
	sphere::Sphere,
	triangle::Triangle,
	vec3::Vec3,
};

pub struct Scene {
	objects: Vec<Object>,
	emitters: Vec<Emitter>,
	sounds: Vec<Sound>,
	receivers: Vec<Receiver>,
}

#[derive(Debug, PartialEq)]
pub struct Emitter {
	pub origin: Vec3,
	pub data: Vec<f32>,
	pub sounds_per_tick: usize,
}

pub enum Receiver {
	Spherical(Sphere),
}

pub struct Object {
	geometry: Vec<Triangle<Vec3>>,
	reflectance: f32,
	transmittance: f32,
	absorbance: f32,
}

impl Object {
	pub fn new(
		geometry: Vec<Triangle<Vec3>>,
		reflectance: f32,
		transmittance: f32,
		absorbance: f32,
	) -> Self {
		Self {
			geometry,
			reflectance,
			transmittance,
			absorbance,
		}
	}
}

pub fn build_geometry_from_triangle_fan(points: Vec<Vec3>) -> Vec<Triangle<Vec3>> {
	let origin: &Vec3 = &points[0];
	let mut prev: &Vec3 = &points[1];

	points
		.iter()
		.skip(2)
		.map(|point| -> Triangle<Vec3> {
			let triangle: Triangle<Vec3> = Triangle(*origin, *prev, *point);
			prev = point;
			triangle
		})
		.collect()
}

#[test]
fn triangle_fan_three_points() {
	assert_eq!(
		build_geometry_from_triangle_fan(vec![
			Vec3(0_f32, 0_f32, 0_f32),
			Vec3(0_f32, 1_f32, 0_f32),
			Vec3(1_f32, 1_f32, 0_f32)
		]),
		vec![Triangle(
			Vec3(0_f32, 0_f32, 0_f32),
			Vec3(0_f32, 1_f32, 0_f32),
			Vec3(1_f32, 1_f32, 0_f32)
		)]
	)
}

#[test]
fn triangle_fan_four_points_two_triangles() {
	assert_eq!(
		build_geometry_from_triangle_fan(vec![
			Vec3(0_f32, 0_f32, 0_f32),
			Vec3(0_f32, 1_f32, 0_f32),
			Vec3(1_f32, 1_f32, 0_f32),
			Vec3(1_f32, 0_f32, 0_f32)
		]),
		vec![
			Triangle(
				Vec3(0_f32, 0_f32, 0_f32),
				Vec3(0_f32, 1_f32, 0_f32),
				Vec3(1_f32, 1_f32, 0_f32)
			),
			Triangle(
				Vec3(0_f32, 0_f32, 0_f32),
				Vec3(1_f32, 1_f32, 0_f32),
				Vec3(1_f32, 0_f32, 0_f32)
			)
		]
	)
}

pub type Trace = Vec<Hit>;

pub struct Sound {
	ray: Ray,
	origin_time: f32,
	trace: Trace,
	data: Vec<f32>,
}

impl Scene {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn emitter(mut self, emitter: Emitter) -> Scene {
		self.emitters.push(emitter);
		self
	}

	pub fn receiver(mut self, receiver: Receiver) -> Scene {
		self.receivers.push(receiver);
		self
	}

	pub fn object(mut self, object: Object) -> Scene {
		self.objects.push(object);
		self
	}

	pub fn emitters(&self) -> &Vec<Emitter> {
		&self.emitters
	}

	pub fn receive(&mut self) -> Vec<Vec<f32>> {
		loop {
			self.emitters().iter().for_each(|emitter| {
				println!("Emitter {:?} emitting...", emitter);

				let sounds_to_emit: usize = emitter.sounds_per_tick;

				for i in 0..sounds_to_emit {
					println!("Emitting sound {}", i);
				}
			});

			break;
		}

		// Emitters emit sounds
		// For each sound
		// - Compute all intersections with all objects in the Scene (stored in VecDeque)
		// - If no intersections (all None), kill the sound
		// - Otherwise, sound "bounces off" nearest intersection, or gets read by the receiver
		// Receivers keep a Vec of hits

		vec![vec![]]
	}
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			objects: Vec::default(),
			emitters: Vec::default(),
			sounds: Vec::default(),
			receivers: Vec::default(),
		}
	}
}
