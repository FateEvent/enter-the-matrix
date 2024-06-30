use std::panic;
use matrix::linear_algebra::matrix::Matrix;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", z);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		println!("{}\n", z.row_echelon_form());

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		println!("{}\n", z.reduced_row_echelon_form());

		let u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", u);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		println!("{}\n", u.row_echelon_form());

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		println!("{}\n", u.reduced_row_echelon_form());

		let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.]]);
		println!("\n\x1b[36mBase Form:\x1b[0m\n{}", v);

		println!("\n\x1b[34mRow Echelon Form:\x1b[0m\n");

		println!("{}\n", v.row_echelon_form());

		println!("\n\x1b[32mReduced Row Echelon Form:\x1b[0m\n");

		println!("{}\n", v.reduced_row_echelon_form());
	});

}
