use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mThe Transpose of a Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let mut z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		z.row_echelon_form();
		z.print();

		let mut u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		println!("u is equal to:\n{}", u);
		u.row_echelon_form();
		u.print();
		let mut v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.]]);
		v.row_echelon_form();
		v.print();
	});

}
