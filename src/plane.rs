use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Plane {
    pub point: Point3,
    pub normal: Direction,
    pub material: Arc<dyn Material>,
    pub offset: f64,
}

impl Plane {
    pub fn new(
        point: Point3,
        mut u: Direction,
        mut v: Direction,
        material: Arc<dyn Material>,
    ) -> Self {
        u = u.normalize();
        v = v.normalize();
        let normal = u.cross(&v).normalize();
        let offset = normal.dot(&point);
        Self {
            point,
            normal,
            material,
            offset,
        }
    }
}

/*
class quad : public hittable {
  public:
    quad(const point3& Q, const vec3& u, const vec3& v, shared_ptr<material> mat)
      : Q(Q), u(u), v(v), mat(mat)
    {
        auto n = cross(u, v);
        normal = unit_vector(n);
        D = dot(normal, Q);

        set_bounding_box();
    }
    ...

  private:
    point3 Q;
    vec3 u, v;
    shared_ptr<material> mat;
    aabb bbox;
    vec3 normal;
    double D;
};
*/
