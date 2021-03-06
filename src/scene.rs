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
		reflectance: f64,
	},
	Receiver {
		geometry: Sphere,
	},
}

impl Object {
	pub fn reflector(geometry: Vec<Triangle<Vec3>>, reflectance: f64) -> Self {
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
	ReceiverHit { hit: Hit, intensity: f64 },
	ObjectHit { hit: Hit, reflectance: f64 },
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
			Vec3(0_f64, 0_f64, 0_f64),
			Vec3(0_f64, 1_f64, 0_f64),
			Vec3(1_f64, 1_f64, 0_f64)
		]),
		vec![Triangle(
			Vec3(0_f64, 0_f64, 0_f64),
			Vec3(0_f64, 1_f64, 0_f64),
			Vec3(1_f64, 1_f64, 0_f64)
		)]
	)
}

#[test]
fn triangle_fan_four_points_two_triangles() {
	assert_eq!(
		build_geometry_from_triangle_fan(vec![
			Vec3(0_f64, 0_f64, 0_f64),
			Vec3(0_f64, 1_f64, 0_f64),
			Vec3(1_f64, 1_f64, 0_f64),
			Vec3(1_f64, 0_f64, 0_f64)
		]),
		vec![
			Triangle(
				Vec3(0_f64, 0_f64, 0_f64),
				Vec3(0_f64, 1_f64, 0_f64),
				Vec3(1_f64, 1_f64, 0_f64)
			),
			Triangle(
				Vec3(0_f64, 0_f64, 0_f64),
				Vec3(1_f64, 1_f64, 0_f64),
				Vec3(1_f64, 0_f64, 0_f64)
			)
		]
	)
}

pub struct Sound {
	ray: Ray,
	intensity: f64,
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

	pub fn simulate(&mut self) -> Vec<(Hit, f64)> {
		use rand::Rng;
		let mut rng = rand::thread_rng();

		const SPEED_OF_SOUND: f64 = 344_f64;

		let mut receiver_hits: Vec<(Hit, f64)> = vec![];

		info!("Beginning simulation...");

		// First, emitters emit sound
		let sounds: Vec<Sound> = self
			.emitters()
			.iter()
			.map(|emitter| -> Vec<Sound> {
				let sounds_to_emit: usize = emitter.sounds_per_tick;

				debug!("Emitting {} sounds...", sounds_to_emit);

				(0..sounds_to_emit)
					.map(|_| {
						use core::f64::consts::PI;

						let theta: f64 = rng.gen_range(0_f64, 2_f64 * PI);

						let phi: f64 = (2_f64 * rng.gen_range(0_f64, 1_f64) - 1_f64).acos();

						let direction: Vec3 = Vec3(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());

						let direction: Vec3 = direction * (SPEED_OF_SOUND / direction.mag());

						Sound {
							ray: Ray::new(emitter.origin, direction),
							intensity: 1_f64,
						}
					})
					.collect()
			})
			.flatten()
			.collect();

		let mut time = 0_f64;
		let mut sounds = sounds;

		loop {
			info!("SOT with {} sounds", sounds.len());

			if sounds.is_empty() {
				break;
			}

			sounds = sounds
				.iter()
				.map(|sound: &Sound| -> Option<Sound> {
					let hits: BTreeSet<Interaction> = self
						.objects
						.iter()
						.map(|object| -> BTreeSet<Interaction> {
							match object {
								Object::Reflector {
									geometry,
									reflectance,
								} => geometry
									.iter()
									.map(|tri| -> Option<Interaction> {
										sound.ray.intersect(tri).map(|hit| Interaction::ObjectHit {
											hit,
											reflectance: *reflectance,
										})
									})
									.filter_map(|x| x)
									.collect(),

								Object::Receiver { geometry } => sound
									.ray
									.intersect(geometry)
									.unwrap_or(vec![])
									.iter()
									.map(|hit| Interaction::ReceiverHit {
										hit: *hit,
										intensity: sound.intensity,
									})
									.collect(),
							}
						})
						.flatten()
						.filter(|hit| -> bool {
							match hit {
								Interaction::ObjectHit { hit, .. } => hit.time > sound.ray.t_offset,
								Interaction::ReceiverHit { hit, .. } => hit.time > sound.ray.t_offset,
							}
						})
						.collect();

					let earliest_hit = hits.iter().nth(0);

					earliest_hit
						.map(|interaction| -> Option<Sound> {
							match interaction {
								Interaction::ObjectHit { hit, reflectance } => {
									let direction: Vec3 = sound.ray.direction
										- 2_f64 * (sound.ray.direction.dot(hit.unit_normal)) * hit.unit_normal;
									let origin: Vec3 = hit.point;
									let t_offset: f64 = hit.time;

									let new_ray: Ray = Ray {
										direction,
										origin,
										t_offset,
									};

									time = hit.time;

									let new_intensity: f64 = sound.intensity * reflectance;

									// If the new intensity isn't super low (near the
									// threshold of human hearing) we should probably
									// just kill it off.
									if new_intensity >= 0.000_000_001 {
										Some(Sound {
											ray: new_ray,
											intensity: new_intensity,
										})
									} else {
										trace!("Killing sound because its amplitude is too low!");
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

			info!("EOT with {} sounds", sounds.len());
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
