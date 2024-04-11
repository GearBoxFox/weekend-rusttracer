use crate::vec3::Vec3;
use crate::ray::Ray;

pub trait Hitable {
    fn hit(&self, ray:&Ray, t_min:f32, t_max:f32) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3,
}

pub struct HitList {
    pub hitable: Vec<Sphere>,
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center:Vec3,
    radius:f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{center: center, radius: radius}
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray:&Ray, t_min:f32, t_max:f32) -> Option<HitRecord> {
        // A is ray origin, B is ray direction, C is sphere origin, t is time, solve for t
        // dot(B, B) * t^2 + 2dot(B, A-C)*t + dot(A-C, A-C) - R*R = 0
        // # of roots = # of hits
        let oc:Vec3 = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            // check for lowest root first
            let mut roots = (-b - discriminant.sqrt()) / (2.0 * a);
            if roots < t_max && roots > t_min {
                let hit_point = ray.point_at_parameter(roots);
                return Some(HitRecord{
                t: roots,
                p: hit_point,
                normal: (hit_point - self.center) / self.radius,
                });
            }
            // check for highest root second
            roots = (-b + discriminant.sqrt()) / (2.0 * a);
            if roots < t_max && roots > t_min {
                let hit_point = ray.point_at_parameter(roots);
                return Some(HitRecord{
                t: roots,
                p: hit_point,
                normal: (hit_point - self.center) / self.radius,
                });
            }
        }
        None
    }
}

impl HitRecord {
    pub fn new (t:f32, p: Vec3, normal:Vec3) -> HitRecord {
        HitRecord{t: t, p: p, normal: normal}
    }
}

impl HitList {
    pub fn new (hitable:Vec<Sphere>) -> HitList {
        HitList{hitable: hitable}
    }

    pub fn hit(&self, ray:&Ray, t_min:f32, t_max:f32) -> Option<HitRecord> {
        // check every hitable object in our list, find the closest if any
        let mut maybe_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        
        for sphere in self.hitable.iter() {
            if let Some(hit) = sphere.hit(&ray, t_min, t_max) {
                closest_so_far = if hit.t < closest_so_far {
                    maybe_hit = Some(hit);
                    hit.t
                } else {
                    closest_so_far
                };
            }
        }
        maybe_hit
    }
}
