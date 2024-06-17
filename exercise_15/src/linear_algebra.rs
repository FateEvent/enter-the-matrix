pub use core::fmt;
pub use core::ops::Neg;
pub use num::complex::Complex;
pub use num::Num;
pub use num::Zero;
pub use num::traits::MulAdd;

use crate::linear_algebra::vector::Vector;
use crate::linear_algebra::matrix::Matrix;

pub mod vector;
pub mod matrix;



pub trait Lerp<V> {
	fn lerp(&self, u: V, v: V, t: f32) -> V;
}

impl Lerp<f32> for f32 {
	fn lerp(&self, u: f32, v: f32, t: f32) -> f32 {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		(1. - t).mul_add(u, t * v)
	}
}

impl Lerp<Vector<f32>> for Vector<f32> {
	fn lerp(&self, u: Vector<f32>, v: Vector<f32>, t: f32) -> Vector<f32> {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		u.mul_add(1.0 - t, &(t * v))
	}
}

impl Lerp<Matrix<f32>> for Matrix<f32> {
	fn lerp(&self, u: Matrix<f32>, v: Matrix<f32>, t: f32) -> Matrix<f32> {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		u.mul_add(1.0 - t, &(t * v))
	}
}

impl Lerp<Complex<f32>> for Complex<f32> {
	fn lerp(&self, u: Complex<f32>, v: Complex<f32>, t: f32) -> Complex<f32> {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		u.mul_add(Complex::<f32>::from(1. - t), t * v)
	}
}



impl Lerp<Vector<Complex<f32>>> for Vector<Complex<f32>> {
	fn lerp(&self, u: Vector<Complex<f32>>, v: Vector<Complex<f32>>, t: f32) -> Vector<Complex<f32>> {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		u.mul_add(Complex::<f32>::from(1.0 - t), &(t * v))
	}
}

impl Lerp<Matrix<Complex<f32>>> for Matrix<Complex<f32>> {
	fn lerp(&self, u: Matrix<Complex<f32>>, v: Matrix<Complex<f32>>, t: f32) -> Matrix<Complex<f32>> {
		if t < 0.0 || t > 1.0 {
			panic!("t must be comprised between 0 and 1");
		}
		u.mul_add(Complex::<f32>::from(1.0 - t), &(t * v))
	}
}

pub fn lerp<V: Clone>(u: V, v: V, t: f32) -> V
where V: Lerp<V>,
{
	u.lerp(u.clone(), v, t)
}

pub trait AngleCos<K> {
	fn angle_cos(&self, u: Vector<K>, v: Vector<K>) -> f32;
}

impl AngleCos<f32> for Vector<f32> {
	fn angle_cos(&self, u: Vector<f32>, v: Vector<f32>) -> f32 {
		u.dot(v.clone()) / (u.norm() * v.norm())
	}
}

impl AngleCos<Complex<f32>> for Vector<Complex<f32>> {
	fn angle_cos(&self, u: Vector<Complex<f32>>, v: Vector<Complex<f32>>) -> f32 {
		u.dot(v.clone()) / (u.norm() * v.norm())
	}
}

pub fn angle_cos<K: Clone>(u: Vector<K>, v: Vector<K>) -> f32
where Vector<K>: AngleCos<K> {
	u.angle_cos(u.clone(), v)
}

pub trait CrossProduct<K> {
	fn cross_product(&self, u: Vector<K>, v: Vector<K>) -> Vector<K>;
}

impl CrossProduct<f32> for Vector<f32> {
	fn cross_product(&self, u: Vector<f32>, v: Vector<f32>) -> Vector<f32> {
		if u.get_rows() != 3 || v.get_rows() != 3 {
			panic!("Vectors must have three dimensions.");
		};
		let mut cross = Vector::new();
		cross.push(u[1].mul_add(v[2], -(u[2] * v[1])));
		cross.push(u[2].mul_add(v[0], -(u[0] * v[2])));
		cross.push(u[0].mul_add(v[1], -(u[1] * v[0])));

		cross
	}
}

impl CrossProduct<Complex<f32>> for Vector<Complex<f32>> {
	fn cross_product(&self, u: Vector<Complex<f32>>, v: Vector<Complex<f32>>) -> Vector<Complex<f32>> {
		if u.get_rows() != 3 || v.get_rows() != 3 {
			panic!("Vectors must have three dimensions.");
		};
		let mut cross = Vector::new();
		cross.push(u[1].mul_add(v[2], -(u[2] * v[1])));
		cross.push(u[2].mul_add(v[0], -(u[0] * v[2])));
		cross.push(u[0].mul_add(v[1], -(u[1] * v[0])));

		cross
	}
}

pub fn cross_product<K: Clone>(u: Vector<K>, v: Vector<K>) -> Vector<K>
where Vector<K>: CrossProduct<K> {
	u.cross_product(u.clone(), v)
}
