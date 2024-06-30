use std::panic;
use crate::linear_algebra::vector::Vector;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mMatrix Multiplication:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[5., 7.], &[3., 4.]]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_mat(v));
		// [46 63]
		// [27 37]
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[5., 7.], &[3., 4.], &[5., 22.]]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_mat(v));
		// [366 1471]
		// [117 433]
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[1., 3.], &[2., 4.], &[2., 5.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[1., 3., 2., 2.], &[2., 4., 5., 1.]]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_mat(v));
		// [7 15 17 5]
		// [10 22 24 8]
		// [12 26 29 9]
	});

	println!("\n\x1b[32mInvalid Dimensions - Multiplication is not Defined:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 6.]]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_mat(v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
		println!("u is equal to:\n{}", u);
		let v = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 22., 45.]]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_mat(v));
	});

	println!("\n\x1b[32mMatrix - Vector Multiplication:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[4., 3., 1.], &[6., 7., 2.]]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from(&[3., 1., 4.]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_vec(v));
		// [19]
		// [33]
	});

	println!("\n\x1b[32mInvalid Dimensions - Multiplication is not Defined:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from(&[&[4., 3., 1.], &[6., 7., 2.]]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from(&[3., 1., 4., 5.]);
		println!("v is equal to:\n{}", v);
		println!("u multiplied by v is equal to:\n{}\n", u.mul_vec(v));
	});
}
