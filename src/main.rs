use acray::{
	build_geometry_from_triangle_fan, Emitter, Intersect, Object, Ray, Receiver, Scene, Sphere,
	Triangle, Vec3,
};

fn main() {
	let emitter: Emitter = Emitter {
		origin: Vec3(1.0, 0.0, 0.0),
		data: vec![0_f32, 1_f32, 0_f32, -1_f32],
		start_time: 0.0_f32,
		end_time: 1.0_f32,
		tick_length: 1.0_f32,
		sounds_per_tick: 100,
	};

	let receiver: Receiver = Receiver::Spherical(Sphere::new(Vec3(0_f32, 0_f32, 0_f32), 0.1_f32));

	let wall: Object = Object::new(
		build_geometry_from_triangle_fan(vec![
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
		0_f32,
		0_f32,
	);

	let mut scene: Scene = Scene::new()
		.emitter(emitter)
		.receiver(receiver)
		.object(wall);

	println!("Starting simulation...");

	let results: Vec<Vec<f32>> = scene.receive();

	println!("Simulation complete!");
}
