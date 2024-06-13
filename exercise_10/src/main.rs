use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let mut z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", z);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		z.row_echelon_form();
		z.print();

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		z.reduced_row_echelon_form();
		z.print();

		let mut u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", u);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		u.row_echelon_form();
		u.print();

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		u.reduced_row_echelon_form();
		u.print();

		let mut v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", v);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		v.row_echelon_form();
		v.print();

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		v.reduced_row_echelon_form();
		v.print();
	});

}
