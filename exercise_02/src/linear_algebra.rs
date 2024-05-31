use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Mul;

use vector::Vector;
// use matrix::Matrix;

pub mod vector;
pub mod matrix;

pub trait ToF64 {
	fn to_f64(&self) -> f64;
}

// Implement the ToF64 trait for f64 itself
impl ToF64 for f64 {
	fn to_f64(&self) -> f64 {
		*self
	}
}

pub trait ToK {
	fn from_f64(value: f64) -> Self;
}

// pub fn lerp<V>(u: V, v: V, t: f64) -> V {
// 	if t < 0.0 || t > 1.0 {
// 		panic!("t must be comprised between 0 and 1");
// 	}
// 	return (1 - t).mul_add(u, t * v);
// }
