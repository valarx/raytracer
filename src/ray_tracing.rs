use crate::material::Material;
use crate::vec_math::{Color, Point3, Vec3};
use rand::prelude::ThreadRng;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord>;
}

//#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: std::rc::Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let mut result = None;
        let mut closest = t_bounds.1;
        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, (t_bounds.0, closest)) {
                closest = hit_record.t;
                result = Some(hit_record)
            }
        }
        result
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl HitRecord {
    pub fn new(
        point: Point3,
        outward_normal: Vec3,
        material: std::rc::Rc<dyn Material>,
        ray: &Ray,
        t: f64,
    ) -> Self {
        let front_face = (ray.direction * outward_normal).is_sign_negative();
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: std::rc::Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: std::rc::Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn is_within_range(t: f64, t_bounds: (f64, f64)) -> bool {
    t >= t_bounds.0 && t <= t_bounds.1
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let half_b = origin_to_center * ray.direction;
        let c = origin_to_center * origin_to_center - self.radius * self.radius;
        let discriminant = 4.0 * half_b * half_b - 4.0 * a * c;
        if discriminant.is_sign_negative() {
            None
        } else {
            let minus_b_to_a = -half_b / a;
            let divided_discriminant = discriminant.sqrt() / (2.0 * a);
            let t = minus_b_to_a - divided_discriminant;
            if is_within_range(t, t_bounds) {
                Some(HitRecord::new(
                    ray.at(t),
                    (ray.at(t) - self.center) / self.radius,
                    std::rc::Rc::clone(&self.material),
                    ray,
                    t,
                ))
            } else {
                let t = minus_b_to_a + divided_discriminant;
                if is_within_range(t, t_bounds) {
                    Some(HitRecord::new(
                        ray.at(t),
                        (ray.at(t) - self.center) / self.radius,
                        self.material.clone(),
                        ray,
                        t,
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, rng: &mut ThreadRng, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            if let Some(record) = scene.hit(self, (0.001, f64::INFINITY)) {
                if let Some(scatter_result) = record.material.scatter(&record, &self, rng) {
                    let new_color = scatter_result.1.color(rng, scene, depth - 1);
                    Vec3::new(
                        scatter_result.0.data[0] * new_color.data[0],
                        scatter_result.0.data[1] * new_color.data[1],
                        scatter_result.0.data[2] * new_color.data[2],
                    )
                } else {
                    Color::new(0.0, 0.0, 0.0)
                }
            } else {
                let unit_direction = self.direction.to_unit();
                let t = 0.5 * (unit_direction.data[1] + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    // w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vector_up: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let h = (fov / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit();
        let u = vector_up.cross_product(w).to_unit();
        let v = w.cross_product(u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left = look_from - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        Camera {
            origin: look_from,
            lower_left,
            horizontal,
            vertical,
            //    w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn create_ray(&self, rng: &mut ThreadRng, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.data[0] + self.v * rd.data[1];
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
