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
    ) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
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
    pub fn new(albedo: Color, mut fuzz: f64) -> Self {
        fuzz = if fuzz > 1. { 1. } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
    ) -> Option<(Ray, Color)> {
        let mut reflected = incident_ray.direction.reflect(normal);
        reflected = reflected.normalize() + self.fuzz * Direction::random_unit();
        let scattered = Ray::new(point.clone(), reflected);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}
