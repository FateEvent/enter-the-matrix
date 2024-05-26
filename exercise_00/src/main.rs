// use std::panic;
use crate::linear_algebra::Vector;
use crate::linear_algebra::Matrix;

pub mod linear_algebra;
 
fn main() {
	let t = Vector::from(&[2., 3.]);
	t.print();
	let u = Vector::from(&[5., 7.]);
	u.print();
	let v = u.add(t);
	v.print();
	let m = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	m.print();
}
