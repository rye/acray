use musac::{Intersect, Ray, Triangle, Vec3};

fn main() {
	let ray = Ray::new(
		Vec3(0.0_f32, 2.0_f32, 0.0_f32),
		Vec3(-0.0_f32, -1.0_f32, 0.0_f32),
	);

	let tri = Triangle(
		Vec3(1.0_f32, 0.0_f32, 0.0_f32),
		Vec3(-1.0_f32, 0.0_f32, 1.0_f32),
		Vec3(-1.0_f32, 0.0_f32, -1.0_f32),
	);

	println!("{:?}", ray.intersect(tri));
}
