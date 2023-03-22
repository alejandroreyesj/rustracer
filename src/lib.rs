pub mod camera;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod units;

//import infinity for f64 and pi
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

#[inline]
pub fn degress_to_radies(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
