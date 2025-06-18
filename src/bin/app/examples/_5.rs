use std::rc::Rc;

use rt::{hittable::HittableList, sphere::Sphere, vec3::Point3};

pub fn ground() -> HittableList {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.1, 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    world
}
