use std::sync::{Arc, OnceLock};

use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::{
    hittables::{Hittable, HittableList, sphere::Sphere},
    materials::Material,
    vec3::{Direction, Point3},
};

type Sampler = dyn Fn(&mut SmallRng) -> f64 + Sync + Send;

static POWER_EDGE_SAMPLER: OnceLock<Box<Sampler>> = OnceLock::new();
static POWER_CENTER_SAMPLER: OnceLock<Box<Sampler>> = OnceLock::new();
static EXP_FALLOFF_SAMPLER: OnceLock<Box<Sampler>> = OnceLock::new();

/// Returns a reference to a power-law sampler biased toward the outer edge.
///
/// Captures `exponent`, e.g. 2.0.
pub fn power_edge_sampler(exponent: f64) -> &'static Sampler {
    POWER_EDGE_SAMPLER
        .get_or_init(|| Box::new(move |rng: &mut SmallRng| rng.random::<f64>().powf(exponent)))
}

/// Returns a reference to a power-law sampler biased toward the center.
///
/// Captures `bias`, e.g. 3.0.
pub fn power_center_sampler(bias: f64) -> &'static Sampler {
    POWER_CENTER_SAMPLER.get_or_init(|| {
        Box::new(move |rng: &mut SmallRng| 1.0 - rng.random::<f64>().powf(1.0 / (bias + 1.0)))
    })
}

/// Returns a reference to an exponential falloff sampler.
///
/// Captures `lambda`, e.g. 1.5, and `swarm_radius`.
pub fn exp_falloff_sampler(lambda: f64, swarm_radius: f64) -> &'static Sampler {
    EXP_FALLOFF_SAMPLER.get_or_init(|| {
        Box::new(move |rng: &mut SmallRng| {
            let u = rng.random::<f64>().max(f64::MIN_POSITIVE); // avoid ln(0)
            let r = -u.ln() / lambda;
            (r.min(swarm_radius)) / swarm_radius
        })
    })
}

pub fn swarm(
    center: Point3,
    swarm_radius: f64,
    particle_radius: f64,
    material: Arc<dyn Material>,
    size: usize,
    sampler: impl Fn(&mut SmallRng) -> f64,
) -> HittableList {
    let base_seed = 12345u64;
    let mut points = Vec::with_capacity(size);

    for i in 0..size {
        let mut rng = SmallRng::seed_from_u64(base_seed + i as u64);
        let direction = Direction::random_unit(&mut rng);
        let distance = swarm_radius * sampler(&mut rng);
        let sphere = Arc::new(Sphere::new(
            center + direction * distance,
            particle_radius,
            material.clone(),
        ));
        points.push(sphere as Arc<dyn Hittable>);
    }

    HittableList { objects: points }
}
