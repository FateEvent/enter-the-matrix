use std::panic;
use crate::linear_algebra::matrix::Matrix;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[32mThe Determinant:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		// let z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		// println!("{}", z.determinant());

		// let u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		// println!("{}", u.determinant());

		// let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.], &[5., 7., 15.]]);
		// println!("{}", v.determinant());
		// // 0
		// v.print();

		// let r = Matrix::from(&[&[5., 7., 15., 18.], &[3., 4., 5., 3.], &[5., 7., 15., 3.], &[1., 2., 1., 3.]]);
		// println!("{}", r.determinant());
		// // 210
		// r.print();

		// let s = Matrix::from(&[&[5., 7., 15., 18., 17.], &[3., 4., 5., 3., 3.], &[5., 7., 15., 3., 7.], &[1., 2., 1., 3., 5.], &[5., 7., 15., 18., 7.]]);
		// println!("{}", s.determinant());
		// s.print();

		// let t = Matrix::from(&[
		// 	&[ 8., 5., -2., 4.],
		// 	&[ 4., 2.5, 20., 4.],
		// 	&[ 8., 5., 1., 4.],
		// 	&[28., -4., 17., 1.],
		// ]);
		// println!("{}", t.determinant());
		// // 1032

		let g = Matrix::from(&[
			&[3., 2., 0.],
			&[0., 0., 1.],
			&[2., -2., 1.]
		]);
		println!("transpose {}", g.transpose());
		// println!("{}", g.determinant());
		let f = g.create_adjoint();
		println!("{}", f);
	});

}
