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

pub fn lerp<V: Clone>(u: V, v: V, t: f32) -> V
where V: Lerp<V>,
{
	u.lerp(u.clone(), v, t)
}  