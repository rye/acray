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
}

#[derive(Debug, PartialEq)]
pub struct Emitter {
	pub origin: Vec3,
	pub sounds_per_tick: usize,
}

pub enum Object {
	Reflector {
		geometry: Vec<Triangle<Vec3>>,
		reflectance: f32,
	},
	Receiver {
		geometry: Sphere,
	},
}

impl Object {
	pub fn reflector(geometry: Vec<Triangle<Vec3>>, reflectance: f32) -> Self {
		Self::Reflector {
			geometry,
			reflectance,
		}
	}

	pub fn receiver(geometry: Sphere) -> Self {
		Self::Receiver { geometry }
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Interaction {
	ReceiverHit { hit: Hit, intensity: f32 },
	ObjectHit { hit: Hit, reflectance: f32 },
}

use core::cmp::Ordering;

impl core::cmp::PartialOrd for Interaction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Self::ObjectHit { hit: hit_a, .. }, Self::ObjectHit { hit: hit_b, .. }) => {
				Some(hit_a.cmp(hit_b))
			}
			(Self::ReceiverHit { hit: hit_a, .. }, Self::ReceiverHit { hit: hit_b, .. }) => {
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

pub struct Sound {
	ray: Ray,
	intensity: f32,
}

impl Scene {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn emitter(mut self, emitter: Emitter) -> Scene {
		self.emitters.push(emitter);
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

	pub fn simulate(&mut self) -> Vec<(Hit, f32)> {
		use rand::Rng;
		let mut rng = rand::thread_rng();

		const SPEED_OF_SOUND: f32 = 344_f32;

		let mut receiver_hits: Vec<(Hit, f32)> = vec![];

		trace!("Beginning simulation...");

		// First, emitters emit sound
		let sounds: Vec<Sound> = self
			.emitters()
			.iter()
			.map(|emitter| -> Vec<Sound> {
				let sounds_to_emit: usize = emitter.sounds_per_tick;

				debug!("Emitting {} sounds...", sounds_to_emit);

				(0..sounds_to_emit)
					.map(|_| {
						let direction: Vec3 = Vec3(rng.gen(), rng.gen(), rng.gen());
						let direction: Vec3 = direction * (SPEED_OF_SOUND / direction.mag());

						Sound {
							ray: Ray::new(emitter.origin, direction),
							intensity: 1_f32,
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
					let hits: BTreeSet<Interaction> = self
						.objects
						.iter()
						.map(|object| -> Option<Interaction> {
							match object {
								Object::Reflector {
									geometry,
									reflectance,
								} => {
									let set: BTreeSet<Interaction> = geometry
										.iter()
										.map(|tri| -> Option<Interaction> {
											sound.ray.intersect(tri).map(|hit| Interaction::ObjectHit {
												hit,
												reflectance: *reflectance,
											})
										})
										.filter_map(|x| x)
										.collect();

									set.iter().take(1).next().cloned()
								}

								Object::Receiver { geometry } => {
									let set: BTreeSet<Interaction> = sound
										.ray
										.intersect(geometry)
										.unwrap_or(vec![])
										.iter()
										.map(|hit| Interaction::ReceiverHit {
											hit: *hit,
											intensity: sound.intensity,
										})
										.collect();

									set.iter().take(1).next().cloned()
								}
							}
						})
						.flatten()
						.collect();

					// let object_hits: BTreeSet<Option<Interaction>> = self
					// 	.objects
					// 	.iter()
					// 	.map(|object: &Object| -> Vec<Option<Interaction>> {
					// 		object
					// 			.geometry
					// 			.iter()
					// 			.map(|tri: &Triangle<Vec3>| -> Option<Interaction> {
					// 				sound
					// 					.ray
					// 					.intersect(tri)
					// 					.map(|hit: Hit| Interaction::ObjectHit {
					// 						hit,
					// 						reflectance: object.reflectance,
					// 					})
					// 			})
					// 			.collect()
					// 	})
					// 	.flatten()
					// 	.collect();

					// let receiver_hits: BTreeSet<Option<Interaction>> = self
					// 	.receivers
					// 	.iter()
					// 	.map(|receiver: &Receiver| -> Option<Vec<Interaction>> {
					// 		match receiver {
					// 			Receiver::Spherical(sphere) => sound.ray.intersect(sphere).map(|hits: Vec<Hit>| {
					// 				hits
					// 					.iter()
					// 					.map(|hit: &Hit| Interaction::ReceiverHit { hit: *hit, amplitude: sound.amplitude })
					// 					.collect()
					// 			}),
					// 		}
					// 	})
					// 	.flatten()
					// 	.flatten()
					// 	.map(Some)
					// 	.collect();

					let earliest_hit = hits
						.iter()
						.filter(|hit| -> bool {
							match hit {
								Interaction::ObjectHit { hit, .. } => hit.time > sound.ray.t_offset,
								Interaction::ReceiverHit { hit, .. } => hit.time > sound.ray.t_offset,
							}
						})
						.take(1)
						.next();

					earliest_hit
						.map(|interaction| -> Option<Sound> {
							match interaction {
								Interaction::ObjectHit { hit, reflectance } => {
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

									let new_amplitude: f32 = sound.intensity * reflectance;
									if new_amplitude >= 0.000_000_000_001 {
										Some(Sound {
											ray: new_ray,
											intensity: new_amplitude,
										})
									} else {
										None
									}
								}
								Interaction::ReceiverHit { hit, intensity } => {
									receiver_hits.push((*hit, *intensity));
									None
								}
							}
						})
						.flatten()
				})
				.filter_map(|x| x)
				.collect();

			trace!("End of tick, still have {} sounds", sounds.len());
		}

		receiver_hits
	}
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			objects: Vec::default(),
			emitters: Vec::default(),
			sounds: Vec::default(),
		}
	}
}
