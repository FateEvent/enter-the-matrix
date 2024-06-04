use std::panic;
use crate::linear_algebra::vector::Vector;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
	// println!("\n\x1b[32mCross Product:\x1b[0m\n");
	// let _ = panic::catch_unwind(|| {
	// 	let u: Matrix<f32> = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	let v = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	// 	println!("v is equal to:\n{}", v);
	// 	println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	// });

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[5., 7.], &[3., 4.], &[5., 22.]]);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	});

	// let _ = panic::catch_unwind(|| {
	// 	let u = Vector::from(&[1., 2., 3.]);
	// 	println!("u is equal to:\n{}", u);
	// 	let v = Vector::from(&[4., 5., 6.]);
	// 	println!("v is equal to:\n{}", v);
	// 	println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Vector::from(&[4., 2., -3.]);
	// 	println!("u is equal to:\n{}", u);
	// 	let v = Vector::from(&[-2., -5., 16.]);
	// 	println!("v is equal to:\n{}", v);
	// 	println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	// });

	// println!("\n\x1b[32mInvalid Matrices:\x1b[0m\n");
	// let _ = panic::catch_unwind(|| {
	// 	let u: Matrix<f32> = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	let v = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 6.]]);
	// 	println!("v is equal to:\n{}", v);
	// 	println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	let v = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 22., 45.]]);
	// 	println!("v is equal to:\n{}", v);
	// 	println!("The cross product of u × v is:\n{}\n", u.mul_mat(v));
	// });
}
