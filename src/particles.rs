use std::sync::Arc;

use rand::{SeedableRng, rngs::SmallRng};

use crate::{
    hittables::{Hittable, HittableList, sphere::Sphere},
    materials::Material,
    vec3::{Direction, Point3},
};

pub fn swarm(
    center: Point3,
    swarm_radius: f64,
    particle_radius: f64,
    material: Arc<dyn Material>,
    size: usize,
    density: f64,
) -> HittableList {
    let base_seed = 12345u64;
    let mut points = Vec::new();

    for i in 0..size {
        let mut rng = SmallRng::seed_from_u64(base_seed + i as u64);
        let direction = Direction::random_unit(&mut rng);
        let distance = swarm_radius * rand::random::<f64>().powf(density);
        let sphere = Arc::new(Sphere::new(
            center + direction * distance,
            particle_radius,
            material.clone(),
        ));
        points.push(sphere as Arc<dyn Hittable>);
    }

    HittableList { objects: points }
}
