use crate::linear_algebra::vector::Vector;
use crate::linear_algebra::matrix::Matrix;

pub mod vector;
pub mod matrix;

// impl<V> lerp for Vector<f32> {
// 	type Output = Vector<f32>;

// 	fn lerp(self, _rhs: Vector<f32>) -> Vector<f32> {
// 		self.vectors_have_equal_length(_rhs.clone());
// 		let mut diff: Vector<f32> = Vector::new();
// 		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
// 			diff.values.push(a - b);
// 			diff.rows += 1;
// 		}
// 		diff
// 	}
// }

pub fn lerp<V>(u: V, v: V, t: f32) -> V {
    if t < 0.0 || t > 1.0 {
        panic!("t must be comprised between 0 and 1");
    }
    return (1. - t) * u + t * v;
}
