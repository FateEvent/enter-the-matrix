use std::panic;
use crate::linear_algebra::matrix::Matrix;
use crate::linear_algebra::projection;
use linear_algebra::PI;

pub mod linear_algebra;

const NEAR_Z: f32 = 0.5;
const FAR_Z: f32 = 2.5;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[36mThe Projection Matrix:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		let width = 2000.;
		let height = 1024.;
		let half_width = width * 0.25;
	    let half_height = height * 0.25;
    	let aspect = width / height;

		let P: Matrix<f32> = projection(60.0 * (PI / 180.0), aspect, 1., 10.);
		P.print();

	});

}
