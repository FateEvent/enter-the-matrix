use std::panic;
use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;
use matrix::linear_algebra::lerp;

fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		let t = Vector::from([2., 3.]);
		t.print();
		let u = Vector::from([5., 7.]);
		u.print();

		println!("{}", Vector::linear_combination(&[t, u], &[4., 7.]));

		println!("\n\x1b[32mLinear Interpolation:\x1b[0m\n");

		println!("{}", lerp(3., 4., 0.5));
		println!("{}", lerp(3., 4., 0.8));
		println!("{}", lerp(21., 42., 0.3));
		println!("{}", lerp(Vector::from([1., 1.]), Vector::from([2., 3.]), 0.8));
		println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
		println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20., 10.], [30., 40.]]), 0.5));
	});
}
