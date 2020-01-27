use acray::{build_geometry_from_triangle_fan, Emitter, Hit, Object, Scene, Sphere, Vec3};

use std::fs::File;

fn main() {
	#[cfg(feature = "simple_logger")]
	simple_logger::init().unwrap();

	let emitter: Emitter = Emitter {
		origin: Vec3(1.0, 0.0, 0.0),
		sounds_per_tick: 1_000_000,
	};

	let receiver: Object = Object::receiver(Sphere::new(Vec3(0_f32, 0_f32, 0_f32), 0.1_f32));

	let front_wall: Object = Object::reflector(
		build_geometry_from_triangle_fan(vec![
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let back_wall: Object = Object::reflector(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, -5_f32),
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let top_wall: Object = Object::reflector(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, 5_f32, 5_f32),
			Vec3(-10.0_f32, 5_f32, 5_f32),
		]),
		0.8_f32,
	);

	let bottom_wall: Object = Object::reflector(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(10.0_f32, 5_f32, -5_f32),
			Vec3(-10.0_f32, 5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let left_wall: Object = Object::reflector(
		build_geometry_from_triangle_fan(vec![
			Vec3(-10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, 5_f32),
			Vec3(10.0_f32, -5_f32, -5_f32),
			Vec3(-10.0_f32, -5_f32, -5_f32),
		]),
		0.8_f32,
	);

	let right_wall: Object = Object::reflector(
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
		.object(receiver)
		.object(front_wall)
		.object(back_wall)
		.object(top_wall)
		.object(bottom_wall)
		.object(left_wall)
		.object(right_wall);

	println!("Starting simulation...");

	let results: Vec<(Hit, f32)> = scene.simulate();
	let file: File = File::create("results.csv").expect("Failed to open results.csv");

	let mut writer = csv::Writer::from_writer(file);

	writer
		.write_record(&["time", "amplitude"])
		.expect("Failed to write headers");

	for (hit, amplitude) in results {
		writer
			.write_record(&[hit.time.to_string(), amplitude.to_string()])
			.expect("Failed to write record");
	}

	writer.flush().expect("Failed to flush the writer");

	println!("Simulation complete!");
}
