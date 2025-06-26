use std::sync::Arc;

use crate::{
    hittables::{disk::Disk, tube::Tube},
    material::Material,
    vec3::{Direction, Point3},
};

pub struct Cylinder {
    pub tube: Box<Tube>,
    pub top: Box<Disk>,
    pub bottom: Box<Disk>,
}

impl Cylinder {
    pub fn new(
        center_of_base: Point3,
        axis: Direction,
        radius: f64,
        material_tube: Arc<dyn Material>,
        material_top: Arc<dyn Material>,
        material_bottom: Arc<dyn Material>,
    ) -> Self {
        assert!(axis.length() > 1e-8, "Axis vector is too small.");
        let [u, v] = orthonormal_basis_2d(&axis.normalize());

        let top = Disk::new(
            center_of_base + axis,
            radius,
            u.clone(),
            v.clone(),
            material_top,
        );
        let bottom = Disk::new(center_of_base, radius, u, v, material_bottom);
        let tube = Tube::new(center_of_base, axis, radius, material_tube);

        Self {
            tube: Box::new(tube),
            top: Box::new(top),
            bottom: Box::new(bottom),
        }
    }
}

fn orthonormal_basis_2d(axis: &Direction) -> [Direction; 2] {
    let w = axis.normalize();
    let a = if w.x.abs() > 0.9 {
        Direction::new(0.0, 1.0, 0.0)
    } else {
        Direction::new(1.0, 0.0, 0.0)
    };

    let v = w.cross(&a).normalize();
    let u = w.cross(&v);
    [u, v]
}
