use std::collections::BTreeSet;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::{
	intersect::{Hit, Intersect},
	products::DotProduct,
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
}

impl Object {
	pub fn new(geometry: Vec<Triangle<Vec3>>, reflectance: f32) -> Self {
		Self {
			geometry,
			reflectance,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Interaction {
	ReceiverHit { hit: Hit },
	ObjectHit { hit: Hit, reflectance: f32 },
}

use core::cmp::Ordering;

impl core::cmp::PartialOrd for Interaction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Self::ObjectHit { hit: hit_a, .. }, Self::ObjectHit { hit: hit_b, .. }) => {
				Some(hit_a.cmp(hit_b))
			}
			(Self::ReceiverHit { hit: hit_a }, Self::ReceiverHit { hit: hit_b }) => {
				Some(hit_a.cmp(hit_b))
			}
			(Self::ObjectHit { hit: hit_a, .. }, Self::ReceiverHit { hit: hit_b, .. }) => {
				Some(hit_a.cmp(hit_b))
			}
			(Self::ReceiverHit { hit: hit_a, .. }, Self::ObjectHit { hit: hit_b, .. }) => {
				Some(hit_a.cmp(hit_b))
			}
		}
	}
}

impl core::cmp::Eq for Interaction {}

impl core::cmp::Ord for Interaction {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::ObjectHit { hit: hit_a, .. }, Self::ObjectHit { hit: hit_b, .. }) => hit_a.cmp(hit_b),
			(Self::ReceiverHit { hit: hit_a, .. }, Self::ReceiverHit { hit: hit_b, .. }) => {
				hit_a.cmp(hit_b)
			}
			(Self::ObjectHit { hit: hit_a, .. }, Self::ReceiverHit { hit: hit_b, .. }) => {
				hit_a.cmp(hit_b)
			}
			(Self::ReceiverHit { hit: hit_a, .. }, Self::ObjectHit { hit: hit_b, .. }) => {
				hit_a.cmp(hit_b)
			}
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
	frequency: f32,
	amplitude: f32,
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
		use rand::Rng;
		let mut rng = rand::thread_rng();

		const SPEED_OF_SOUND: f32 = 344_f32;

		trace!("Beginning simulation...");

		// First, emitters emit sound
		let sounds: Vec<Sound> = self
			.emitters()
			.iter()
			.map(|emitter| -> Vec<Sound> {
				debug!("Emitter {:?} emitting...", emitter);

				let sounds_to_emit: usize = emitter.sounds_per_tick;

				(0..sounds_to_emit)
					.map(|n: usize| {
						trace!("  Emitting sound {}", n);

						let direction: Vec3 = Vec3(rng.gen(), rng.gen(), rng.gen());
						let direction: Vec3 = direction * (SPEED_OF_SOUND / direction.mag());

						Sound {
							ray: Ray::new(emitter.origin, direction),
							frequency: rng.gen_range(0_f32, 2000_f32),
							amplitude: rng.gen_range(0_f32, 1_f32),
							trace: vec![],
						}
					})
					.collect()
			})
			.flatten()
			.collect();

		let mut time = 0_f32;
		let mut sounds = sounds;

		loop {
			trace!("Start of tick");

			if sounds.is_empty() {
				break;
			}

			sounds = sounds
				.iter()
				.map(|sound: &Sound| -> Option<Sound> {
					let object_hits: BTreeSet<Option<Interaction>> = self
						.objects
						.iter()
						.map(|object: &Object| -> Vec<Option<Interaction>> {
							object
								.geometry
								.iter()
								.map(|tri: &Triangle<Vec3>| -> Option<Interaction> {
									sound
										.ray
										.intersect(tri)
										.map(|hit: Hit| Interaction::ObjectHit {
											hit,
											reflectance: object.reflectance,
										})
								})
								.collect()
						})
						.flatten()
						.collect();

					let receiver_hits: BTreeSet<Option<Interaction>> = self
						.receivers
						.iter()
						.map(|receiver: &Receiver| -> Option<Vec<Interaction>> {
							match receiver {
								Receiver::Spherical(sphere) => sound.ray.intersect(sphere).map(|hits: Vec<Hit>| {
									hits
										.iter()
										.map(|hit: &Hit| Interaction::ReceiverHit { hit: *hit })
										.collect()
								}),
							}
						})
						.flatten()
						.flatten()
						.map(Some)
						.collect();

					let earliest_hit = object_hits
						.union(&receiver_hits)
						.filter(|hit: &&Option<Interaction>| -> bool {
							match hit {
								Some(Interaction::ObjectHit { hit, .. }) => hit.time > sound.ray.t_offset,
								Some(Interaction::ReceiverHit { hit }) => hit.time > sound.ray.t_offset,
								None => false,
							}
						})
						.take(1)
						.next();

					earliest_hit
						.map(|interaction| -> Option<Sound> {
							match interaction {
								Some(Interaction::ObjectHit { hit, reflectance }) => {
									let direction: Vec3 = sound.ray.direction
										- 2_f32 * (sound.ray.direction.dot(hit.unit_normal)) * hit.unit_normal;
									let origin: Vec3 = hit.point;
									let t_offset: f32 = hit.time;

									let new_ray: Ray = Ray {
										direction,
										origin,
										t_offset,
									};

									time = hit.time;

									let new_amplitude: f32 = sound.amplitude * reflectance;
									if new_amplitude >= 0.0001 {
										Some(Sound {
											ray: new_ray,
											frequency: sound.frequency,
											amplitude: new_amplitude,
											// TODO fix -- add trace
											trace: sound.trace.clone(),
										})
									} else {
										None
									}
								}
								Some(Interaction::ReceiverHit { hit, .. }) => {
									debug!("Receiver hit: {:?}", hit);
									None
								}
								None => None,
							}
						})
						.flatten()
				})
				.filter_map(|x| x)
				.collect();

			trace!("End of tick, still have {} sounds", sounds.len());
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
