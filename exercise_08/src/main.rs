use std::panic;
use matrix::linear_algebra::matrix::Matrix;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mThe Trace of a Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[5., 7.,], [3., 4.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// 9
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[5., 7., 64.], [3., 4., 18.], [5., 7., 64.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// 73
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
		// 1
	});

	println!("\n\x1b[32mInvalid Dimensions:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[5., 7., 3., 4.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[5., 7., 64.], [3., 4., 18.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
	});

	let _ = panic::catch_unwind(|| {
		let u = Matrix::from([[1., 3.], [2., 4.], [2., 5.]]);
		println!("u is equal to:\n{}", u);
		println!("The trace of u is equal to: {}\n", u.trace());
	});
}
