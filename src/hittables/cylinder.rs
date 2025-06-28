use std::sync::Arc;

use crate::{
    hittables::{Hittable, HittableList, disk::Disk, tube::Tube},
    materials::Material,
    vec3::{Direction, Point3},
};

pub struct Cylinder {
    pub tube: Arc<Tube>,
    pub top: Arc<Disk>,
    pub bottom: Arc<Disk>,
    pub whole: Arc<dyn Hittable>,
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
        assert!(1e-8 < axis.length(), "Axis vector is too small");
        let [u, v] = orthonormal_basis_2d(&axis.normalize());

        let top = Arc::new(Disk::new(
            center_of_base + axis,
            radius,
            u.clone(),
            v.clone(),
            material_top,
        ));
        let bottom = Arc::new(Disk::new(center_of_base, radius, u, v, material_bottom));
        let tube = Arc::new(Tube::new(center_of_base, axis, radius, material_tube));

        let mut whole = HittableList::new();
        whole.add(top.clone());
        whole.add(bottom.clone());
        whole.add(tube.clone());

        Self {
            tube: tube.clone(),
            top: top.clone(),
            bottom: bottom.clone(),
            whole: Arc::new(whole),
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
