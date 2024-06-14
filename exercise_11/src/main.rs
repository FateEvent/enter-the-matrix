use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[32mThe Determinant:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		// let z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		// println!("\nz is equal to:\n{}", z);

		// println!("{}", z.determinant());
		// z.print();


		// let u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		// println!("\nu is equal to:\n{}", u);

		// println!("{}", u.determinant());
		// u.print();



		let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.], &[5., 7., 15.]]);
		println!("\nv is equal to:\n{}", v);

		println!("{}", v.determinant());
		v.print();

	});

}
