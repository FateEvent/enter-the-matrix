use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mThe Transpose of a Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		let mut u = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
		println!("u is equal to:\n{}", u);
		u.row_swap(0, 1);
		u.print();
		u.row_scl(0, 2.);
		u.print();
		u.add_row_multiple(0, 1, 5.);
		u.print();
		u.print_diagonal();
		let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.]]);
		println!("v is equal to:\n{}", v);
		v.print_diagonal();
		// println!("The transpose of u is equal to:\n{}\n", u.transpose());
		// [5 3]
		// [7 4]
	});

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 7., 64.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	println!("The transpose of u is equal to:\n{}\n", u.transpose());
	// 	// [5 3 5]
	// 	// [7 4 7]
	// 	// [64 18 64]
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	println!("The transpose of u is equal to:\n{}\n", u.transpose());
	// 	// [1 0 0]
	// 	// [0 1 0]
	// 	// [0 0 1]
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[5., 7., 3., 4.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	println!("The transpose of u is equal to:\n{}\n", u.transpose());
	// 	// [5]
	// 	// [7]
	// 	// [3]
	// 	// [4]
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	println!("The transpose of u is equal to:\n{}\n", u.transpose());
	// 	// [5 3]
	// 	// [7 4]
	// 	// [64 18]
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let u = Matrix::from(&[&[1., 3.], &[2., 4.], &[2., 5.]]);
	// 	println!("u is equal to:\n{}", u);
	// 	println!("The transpose of u is equal to:\n{}\n", u.transpose());
	// 	// [1 2 2]
	// 	// [3 4 5]
	// });
}
