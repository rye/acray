use acray::{
	build_geometry_from_triangle_fan, Emitter, Intersect, Object, Ray, Receiver, Scene, Sphere,
	Triangle, Vec3,
};

fn main() {
	#[cfg(feature = "simple_logger")]
	simple_logger::init().unwrap();

	let emitter: Emitter = Emitter {
		origin: Vec3(1.0, 0.0, 0.0),
		sounds_per_tick: 100,
	};

	let receiver: Receiver = Receiver::Spherical(Sphere::new(Vec3(0_f32, 0_f32, 0_f32), 0.1_f32));

	let front_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let back_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, -5_f32),
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let top_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, 5_f32),
		]),
		0.8_f32,
	);

	let bottom_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
			Vec3(-10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let left_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(-10.0_f32, -5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let right_wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, 5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
			Vec3(-10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let mut scene: Scene = Scene::new()
		.emitter(emitter)
		.receiver(receiver)
		.object(front_wall)
		.object(back_wall)
		.object(top_wall)
		.object(bottom_wall)
		.object(left_wall)
		.object(right_wall);

	println!("Starting simulation...");

	let results: Vec<Vec<f32>> = scene.simulate();

	println!("Simulation complete!");
}
