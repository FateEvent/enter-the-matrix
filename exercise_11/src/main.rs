use std::panic;
use std::time::Instant;
use matrix::linear_algebra::matrix::Matrix;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[32mThe Determinant:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		let t: Matrix<f32> = Matrix::from([[1., 0.,], [0., 1.]]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// 1
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// 1
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let t = Matrix::from([[1., 2.,], [3., 4.]]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// -2
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// -2
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let t = Matrix::from([[5., 7., 15.], [3., 4., 5.], [5., 7., 15.]]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// 0
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// 0
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let t = Matrix::from([[5., 7., 15., 18.], [3., 4., 5., 3.], [5., 7., 15., 3.], [1., 2., 1., 3.]]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// 210
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// 210
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let t = Matrix::from([[5., 7., 15., 18., 17.], [3., 4., 5., 3., 3.], [5., 7., 15., 3., 7.], [1., 2., 1., 3., 5.], [5., 7., 15., 18., 7.]]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// -2100
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// -2100
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let t = Matrix::from([
			[ 8., 5., -2., 4.],
			[ 4., 2.5, 20., 4.],
			[ 8., 5., 1., 4.],
			[28., -4., 17., 1.],
		]);
		let now = Instant::now();
		println!("{}", t.determinant());
		// 1032
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);

		let now = Instant::now();
		println!("{}", t.determinant_by_row_reduction());
		// 1032
		let elapsed = now.elapsed();
		println!("Elapsed: {:.2?}", elapsed);
	});
}
