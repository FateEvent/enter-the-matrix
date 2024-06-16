use std::panic;
use crate::linear_algebra::Vector;
use crate::linear_algebra::Complex;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4mThe Norm\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let complex_numbers = [
			Complex::new(1.0, 1.0),
			Complex::new(1.0, -1.0)
		];

		let vec: Vec<Complex<f32>> = complex_numbers.to_vec();

		let mut t = Vector::from_vec(vec);
		t.print();
	});
}
