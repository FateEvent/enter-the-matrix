use std::panic;
use crate::linear_algebra::matrix::Matrix;
use crate::linear_algebra::projection;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[36mThe Projection Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let P: Matrix<f32> = projection(45.0, 1.2, 0.1, 100.);
		P.write_matrix_to_file("../varia/matrix_display/proj");
		P.print();
	});

}
