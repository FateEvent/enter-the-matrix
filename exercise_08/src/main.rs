use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mThe Trace of a Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// 9
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 7., 64.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// 73
	});

	println!("\n\x1b[32mInvalid Dimensions:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 3., 4.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[1., 3.], &[2., 4.], &[2., 5.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// [7 15 17 5]
		// [10 22 24 8]
		// [12 26 29 9]
	});
}
