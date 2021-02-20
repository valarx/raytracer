use rand::prelude::ThreadRng;
use rand::Rng;
use std::{cmp::Ordering, ops};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub data: [f64; 3],
}

pub fn random_double_in_interval(rng: &mut ThreadRng, interval: (f64, f64)) -> f64 {
    interval.0 + (interval.1 - interval.0) * rng.gen::<f64>()
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3 {
        Vec3 {
            data: [rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()],
        }
    }

    pub fn random_in_interval(rng: &mut ThreadRng, interval: (f64, f64)) -> Vec3 {
        Vec3 {
            data: [
                random_double_in_interval(rng, interval),
                random_double_in_interval(rng, interval),
                random_double_in_interval(rng, interval),
            ],
        }
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let random_vector = Vec3::random_in_interval(rng, (-1.0, 1.0));
            if random_vector.len_squared().partial_cmp(&1.0).unwrap() == std::cmp::Ordering::Less {
                return random_vector;
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let random_vector = Vec3::new(
                random_double_in_interval(rng, (-1.0, 1.0)),
                random_double_in_interval(rng, (-1.0, 1.0)),
                0.0,
            );
            if random_vector.len_squared() < 1.0 {
                return random_vector;
            }
        }
    }

    pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3 {
        let random_in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if (random_in_unit_sphere * normal).partial_cmp(&0.0).unwrap() == Ordering::Greater {
            random_in_unit_sphere
        } else {
            return -random_in_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        let sigma = 1e-8;
        return self.data[0].abs().partial_cmp(&sigma).unwrap() == Ordering::Less
            && self.data[1].abs().partial_cmp(&sigma).unwrap() == Ordering::Less
            && self.data[2].abs().partial_cmp(&sigma).unwrap() == Ordering::Less;
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn cross_product(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        )
    }

    pub fn to_unit(&self) -> Vec3 {
        let len = self.len();
        *self / len
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (2.0 * (*self) * (*normal)) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, refraction_ratio: f64) -> Vec3 {
        let cos_theta = (-*self * (*normal)).min(1.0);
        let r_out_perp = refraction_ratio * (*self + cos_theta * (*normal));
        let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs().sqrt()) * (*normal);
        r_out_perp + r_out_parallel
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        )
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.data[0], self * rhs.data[1], self * rhs.data[2])
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> f64 {
        self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2]
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3::new(-self.data[0], -self.data[1], -self.data[2])
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
