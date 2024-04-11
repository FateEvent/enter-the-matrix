// use std::panic;
use crate::linear_algebra::Vector;
use crate::linear_algebra::Matrix;

pub mod linear_algebra;
 
fn main() {
	let u = Vector::from(&[2., 3.]);
	u.print();
	let v = Vector::from(&[5., 7.]);
	v.print();
	let x = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	x.print();
}
