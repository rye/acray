use std::collections::BTreeSet;

use log::{error, debug, info, trace, warn};

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

impl Intersectable for Object {}
impl Intersectable for Receiver {}

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
	trace: Trace,
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

	pub fn sound(mut self, sound: Sound) {
		self.sounds.push(sound);
	}

	pub fn emitters(&self) -> &Vec<Emitter> {
		&self.emitters
	}

	pub fn sounds(&self) -> &Vec<Sound> {
		&self.sounds
	}

	pub fn simulate(&mut self) -> Vec<Vec<f32>> {
		const SPEED_OF_SOUND: f32 = 344_f32;

		trace!("Beginning simulation...");

		// First, emitters emit sound
		let sounds: Vec<Sound> = self.emitters().iter().map(|emitter| -> Vec<Sound> {
			debug!("Emitter {:?} emitting...", emitter);

			let sounds_to_emit: usize = emitter.sounds_per_tick;

			(0..sounds_to_emit).map(|n: usize| {
				trace!("  Emitting sound {}", n);
				Sound {
					ray: Ray::new(emitter.origin,
						Vec3(0_f32, 0_f32, 0_f32)
					),
					trace: vec![],
				}
			}).collect()
		}).flatten().collect();

		let mut sounds = sounds;

		loop {
			trace!("Start of tick");

			if sounds.len() == 0 {
				break;
			}

			sounds.iter_mut().for_each(|sound: &mut Sound| {
				let object_hits: BTreeSet<Option<Hit>> = self.objects.iter().map(|object: &Object| -> Vec<Option<Hit>> {
					object.geometry.iter().map(|tri: &Triangle<Vec3>| -> Option<Hit> {
						sound.ray.intersect(tri)
					}).collect()
				}).flatten().collect();

				let receiver_hits: BTreeSet<Option<Hit>> = self.receivers.iter().map(|receiver: &Receiver| -> Vec<Option<Hit>> {
					vec![None]
				}).flatten().collect();

				debug!("  Object hits: {:?}", object_hits);
				debug!("Receiver hits: {:?}", receiver_hits);

				// object_hits.iter().chain(receiver_hits.iter()).map(Clone::clone).collect()
			});

			trace!("End of tick.");
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
