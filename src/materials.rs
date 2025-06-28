use rand::{Rng, rngs::SmallRng};

use crate::{
    color::Color,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        front_face: bool,
        rngs: &mut SmallRng,
    ) -> Option<(Ray, Color)>;

    fn emit(&self, _point: &Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        _front_face: bool,
        rng: &mut SmallRng,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = *normal + Direction::random_unit(rng);
        if scatter_direction.near_zero() {
            scatter_direction = normal.clone();
        }
        let scattered = Ray::new(point.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // in the range [0.0, 1.0]
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        _front_face: bool,
        rng: &mut SmallRng,
    ) -> Option<(Ray, Color)> {
        let mut reflected = incident_ray.direction.reflect(normal);
        reflected = reflected.normalize() + self.fuzz * Direction::random_unit(rng);
        if reflected.near_zero() {
            reflected = normal.clone();
        }
        let scattered = Ray::new(point.clone(), reflected);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        assert!(1e-8 < refraction_index, "Refraction index is too small");
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        front_face: bool,
        rng: &mut SmallRng,
    ) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);

        let refraction_index = if front_face {
            self.refraction_index
        } else {
            1. / self.refraction_index
        };

        let unit_direction = incident_ray.direction.normalize();
        let cos_theta = -unit_direction.dot(normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.;

        let mut direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_index) > rng.random_range(0.0..1.0)
        {
            unit_direction.reflect(normal)
        } else {
            unit_direction.refract(normal, refraction_index)
        };

        if direction.near_zero() {
            direction = *normal;
        }
        let scattered = Ray::new(*point, direction);
        Some((scattered, attenuation))
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schick approximation
        let mut r_0 = (1. - refraction_index) / (1. + refraction_index);
        r_0 *= r_0;
        r_0 + (1. - r_0) * (1. - cosine).powf(5.)
    }
}

pub struct Light {
    pub color: Color,
}

impl Material for Light {
    fn scatter(
        &self,
        _incident_ray: &Ray,
        _point: &Point3,
        _normal: &Direction,
        _front_face: bool,
        _rng: &mut SmallRng,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emit(&self, _point: &Point3) -> Color {
        self.color.clone()
    }
}

impl Light {
    pub fn new(color: Color) -> Self {
        Light { color }
    }
}

pub struct Isotropic {
    albedo: Color,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _incident_ray: &Ray,
        point: &Point3,
        _normal: &Direction,
        _front_face: bool,
        rng: &mut SmallRng,
    ) -> Option<(Ray, Color)> {
        let scattered = Ray::new(*point, Direction::random_unit(rng));
        Some((scattered, self.albedo.clone()))
    }
}
