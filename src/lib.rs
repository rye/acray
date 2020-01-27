mod intersect;
mod products;
mod ray;
mod scene;
mod sphere;
mod triangle;
mod vec3;

pub use intersect::{Hit, Intersect};
pub use products::{CrossProduct, DotProduct};
pub use ray::Ray;
pub use scene::{build_geometry_from_triangle_fan, Emitter, Object, Receiver, Scene, Sound};
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use vec3::Vec3;
