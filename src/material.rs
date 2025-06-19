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

struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        incident_ray: &Ray,
        point: &Point3,
        normal: &Direction,
    ) -> Option<(Ray, Color)> {
        let reflected = incident_ray.direction.reflect(normal);
        let scattered = Ray::new(point.clone(), reflected);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

// bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
//     const override {
//         vec3 reflected = reflect(r_in.direction(), rec.normal);
//         scattered = ray(rec.p, reflected);
//         attenuation = albedo;
//         return true;
//     }
