use crate::linear_algebra::vector::Vector;
// use crate::linear_algebra::matrix::Matrix;

pub mod vector;
// pub mod matrix;
// pub mod lerp;

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
		(1.0 - t) * u + t * v
	}
}

pub fn lerp<V: Clone>(u: V, v: V, t: f32) -> V
where V: Lerp<V>,
{
    u.lerp(u.clone(), v, t)
}
