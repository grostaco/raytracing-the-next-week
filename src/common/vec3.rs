use std::{ops::{Add, Div, Mul, Neg, Sub, Range}, io::{self, Write}};

use rand::{Rng, thread_rng};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

macro_rules! ops_impl_for {
    ($op:ty => $block:tt, $t:ty) => {
        impl $op for $t $block
    };
    ($op:ty => $block:tt, $t:ty, $($ts:ty),+) => {
        impl $op for $t $block
        ops_impl_for!($op => $block, $($ts),* );
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(e0, e1, e2)
    }

    pub fn random<R: Rng>(rng: R) -> Self {
        Self::random_range(rng, 0.0..1.0)
    }

    pub fn random_range<R: Rng>(mut rng: R, range: Range<f64>) -> Self {
        Self(rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range))
    }

    pub fn zeros() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - &(n * self.dot(n) * 2.0)
    }

    pub fn get_color_buf(&self, samples_per_pixel: u32) -> String {
        let mut r = self.0;
        let mut g = self.1;
        let mut b = self.2;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
        
        format!(
            "{} {} {}\n",
            (r.clamp(0.0, 0.999) * 256.0) as u32,
            (g.clamp(0.0, 0.999) * 256.0) as u32,
            (b.clamp(0.0, 0.999) * 256.0) as u32
        )
    }

    pub fn write_color<W: Write>(&self, mut image: W, samples_per_pixel: u32) -> io::Result<()> {
        let mut r = self.0;
        let mut g = self.1;
        let mut b = self.2;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
        
        write!(
            image,
            "{} {} {}\n",
            (r.clamp(0.0, 0.999) * 256.0) as u32,
            (g.clamp(0.0, 0.999) * 256.0) as u32,
            (b.clamp(0.0, 0.999) * 256.0) as u32
        )?;
        Ok(())
    }
    

    pub fn x(&self) -> f64 {
        self.0
    }
    /* 
    fn z(&self) -> f64 {
        self.2
    }
    */
    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(self.1 * rhs.2 - self.2 * rhs.1, self.2 * rhs.0 - self.0 * rhs.2, self.0 * rhs.1 - self.1 * rhs.0)
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = thread_rng();
        loop {
            let p = Self::random_range(&mut rng, -1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 { return p; }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn near_zero(&self) -> bool {
        const E: f64 = 1e-8;
        self.0.abs() < E && self.1.abs() < E && self.2.abs() < E
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = self.dot(&-n).min(1.0);
        let r_out_perp = (n * cos_theta + *self) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).sqrt();
        r_out_perp + r_out_parallel
    }
}

ops_impl_for!(Neg => {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}, Vec3, &Vec3);

ops_impl_for!(Mul<f64> => {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(Mul => {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}, Vec3, &Vec3);

ops_impl_for!(Div<f64> => {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(Add => {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)    
    }
}, Vec3, &Vec3);

ops_impl_for!(Sub => {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)    
    }
}, Vec3, &Vec3);

