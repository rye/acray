use crate::{
	intersect::{Hit, Intersect},
	ray::Ray,
	sphere::Sphere,
	triangle::Triangle,
	vec3::Vec3,
};

#[test]
fn intersect_correct_triangle() {
	let ray: Ray = Ray {
		origin: Vec3(0_f64, 0_f64, 0_f64),
		direction: Vec3(1_f64, 0_f64, 0_f64),
		t_offset: 0.0_f64,
	};

	let triangle: Triangle<Vec3> = Triangle(
		Vec3(2_f64, 1_f64, 0_f64),
		Vec3(2_f64, -1_f64, 1_f64),
		Vec3(2_f64, -1_f64, -1_f64),
	);

	let expected_hit_record = Some(Hit {
		time: 2_f64,
		point: Vec3(2_f64, 0_f64, 0_f64),
		unit_normal: Vec3(-1_f64, 0_f64, 0_f64),
	});

	assert_eq!(ray.intersect(&triangle), expected_hit_record);
}

#[test]
fn intersect_correct_triangle_no_intersection() {
	let ray: Ray = Ray {
		origin: Vec3(0_f64, 1_f64, 1_f64),
		direction: Vec3(1_f64, 0_f64, 0_f64),
		t_offset: 0.0_f64,
	};

	let triangle: Triangle<Vec3> = Triangle(
		Vec3(2_f64, 0_f64, 0_f64),
		Vec3(2_f64, 1_f64, 0_f64),
		Vec3(2_f64, 0_f64, 1_f64),
	);

	let expected_hit_record = None;

	assert_eq!(ray.intersect(&triangle), expected_hit_record);
}

#[test]
fn intersect_correct_sphere() {
	let ray: Ray = Ray {
		origin: Vec3(0_f64, 0_f64, 0_f64),
		direction: Vec3(1_f64, 0_f64, 0_f64),
		t_offset: 0.0_f64,
	};

	let sphere: Sphere = Sphere {
		origin: Vec3(2_f64, 0_f64, 0_f64),
		radius: 1_f64,
	};

	let expected_intersections = Some(vec![
		Hit {
			time: 1_f64,
			point: Vec3(1_f64, 0_f64, 0_f64),
			unit_normal: Vec3(-1_f64, 0_f64, 0_f64),
		},
		Hit {
			time: 3_f64,
			point: Vec3(3_f64, 0_f64, 0_f64),
			unit_normal: Vec3(-1_f64, 0_f64, 0_f64),
		},
	]);

	assert_eq!(ray.intersect(&sphere), expected_intersections)
}

#[test]
fn intersect_correct_sphere_tangent() {
	let ray: Ray = Ray {
		origin: Vec3(0_f64, 1_f64, 0_f64),
		direction: Vec3(1_f64, 0_f64, 0_f64),
		t_offset: 0_f64,
	};

	let sphere: Sphere = Sphere {
		origin: Vec3(2_f64, 0_f64, 0_f64),
		radius: 1_f64,
	};

	let expected_intersections = Some(vec![Hit {
		time: 2_f64,
		point: Vec3(2_f64, 1_f64, 0_f64),
		unit_normal: Vec3(0_f64, 1_f64, 0_f64),
	}]);

	assert_eq!(ray.intersect(&sphere), expected_intersections)
}

#[test]
fn intersect_correct_sphere_no_intersection() {
	let ray: Ray = Ray {
		origin: Vec3(0_f64, 2_f64, 0_f64),
		direction: Vec3(1_f64, 0_f64, 0_f64),
		t_offset: 0_f64,
	};

	let sphere: Sphere = Sphere {
		origin: Vec3(2_f64, 0_f64, 0_f64),
		radius: 1_f64,
	};

	let expected_intersections = None;

	assert_eq!(ray.intersect(&sphere), expected_intersections)
}
