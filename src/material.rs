use rand::Rng;

use crate::{
    color::Color,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub trait Material {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        front_face: bool,
    ) -> Option<(Ray, Color)>;
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
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = *normal + Direction::random_unit();
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
    ) -> Option<(Ray, Color)> {
        let mut reflected = incident_ray.direction.reflect(normal);
        reflected = reflected.normalize() + self.fuzz * Direction::random_unit();
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
        Dielectric {
            refraction_index: refraction_index.max(f64::EPSILON),
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
        front_face: bool,
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

        let mut rng = rand::rng();
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_index) > rng.random_range(0.0..1.0)
        {
            unit_direction.reflect(normal)
        } else {
            unit_direction.refract(normal, refraction_index)
        };

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
