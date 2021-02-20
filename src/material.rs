use crate::ray_tracing::{HitRecord, Ray};
use crate::vec_math::{Color, Vec3};
use rand::{prelude::ThreadRng, Rng};

pub trait Material {
    fn scatter(&self, record: &HitRecord, ray: &Ray, rng: &mut ThreadRng) -> Option<(Color, Ray)>;
}

pub struct Diffusor {
    pub color: Color,
}

impl Material for Diffusor {
    fn scatter(&self, record: &HitRecord, _ray: &Ray, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let scatter_direction = Vec3::random_in_hemisphere(rng, record.normal);
        if scatter_direction.near_zero() {
            Some((self.color, Ray::new(record.point, record.normal)))
        } else {
            Some((self.color, Ray::new(record.point, scatter_direction)))
        }
    }
}

pub struct Reflector {
    pub color: Color,
    pub fuzz_coeff: f64,
}

impl Material for Reflector {
    fn scatter(&self, record: &HitRecord, ray: &Ray, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let reflected = ray.direction.to_unit().reflect(&record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + Vec3::random_in_hemisphere(rng, record.normal) * self.fuzz_coeff,
        );
        if scattered.direction * record.normal > 0.0 {
            Some((self.color, scattered))
        } else {
            None
        }
    }
}

pub struct Refractor {
    pub color: Color,
    pub fuzz_coeff: f64,
    pub refr_coeff: f64,
}

fn shlick_approximation_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_squared = r0.powi(2);
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}

impl Material for Refractor {
    fn scatter(&self, record: &HitRecord, ray: &Ray, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refr_coeff
        } else {
            self.refr_coeff
        };
        let unit_direction = ray.direction.to_unit();
        let cos_theta = (-unit_direction * record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        if refraction_ratio * sin_theta > 1.0
            || shlick_approximation_reflectance(cos_theta, refraction_ratio) > rng.gen()
        {
            let reflected = unit_direction.reflect(&record.normal);
            Some((
                self.color,
                Ray::new(
                    record.point,
                    reflected + Vec3::random_in_hemisphere(rng, record.normal) * self.fuzz_coeff,
                ),
            ))
        } else {
            let refracted = unit_direction.refract(&record.normal, refraction_ratio);
            Some((
                self.color,
                Ray::new(
                    record.point,
                    refracted + Vec3::random_in_hemisphere(rng, record.normal) * self.fuzz_coeff,
                ),
            ))
        }
    }
}
