use std::panic;
use matrix::linear_algebra::matrix::Matrix;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[32mThe Inverse of a Matrix:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		let z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
		println!("The determinant is {}", z.determinant());
		println!("{}\n", z.inverse());

		let u = Matrix::from(&[&[1., 2.,], &[3., 4.]]);
		println!("The determinant is {}", u.determinant());
		println!("{}\n", u.inverse());

		let l: Matrix<f32> = Matrix::from(&[&[4., 2.,], &[7., 6.]]);
		println!("The determinant is {}", l.determinant());
		println!("{}\n", l.inverse());

		let r = Matrix::from(&[&[5., 7., 15., 18.], &[3., 4., 5., 3.], &[5., 7., 15., 3.], &[1., 2., 1., 3.]]);
		println!("The determinant is {}", r.determinant());
		println!("{}\n", r.inverse());

		let s = Matrix::from(&[&[5., 7., 15., 18., 17.], &[3., 4., 5., 3., 3.], &[5., 7., 15., 3., 7.], &[1., 2., 1., 3., 5.], &[5., 7., 15., 18., 7.]]);
		println!("The determinant is {}", s.determinant());
		println!("{}\n", s.inverse());

		let t = Matrix::from(&[
			&[ 8., 5., -2., 4.],
			&[ 4., 2.5, 20., 4.],
			&[ 8., 5., 1., 4.],
			&[28., -4., 17., 1.],
		]);
		println!("The determinant is {}", t.determinant());
		println!("{}\n", t.inverse());

		let g = Matrix::from(&[
			&[3., 2., 0.],
			&[0., 0., 1.],
			&[2., -2., 1.]
		]);
		println!("The determinant is {}", g.determinant());
		println!("{}\n", g.inverse());
		// [0.2 -0.2 0.2]
		// [0.2 0.3 -0.3]
		// [0 1 0]

		let h = Matrix::from(&[
			&[8., 5., -2.],
			&[4., 7., 20.],
			&[7., 6., 1.],
		]);
		println!("The determinant is {}", h.determinant());
		println!("{}\n", h.inverse());
		// [0.649425287, 0.097701149, -0.655172414]
		// [-0.781609195, -0.126436782, 0.965517241]
		// [0.143678161, 0.074712644, -0.206896552]

		let u = Matrix::from(&[
			&[2., 0., 0.],
			&[0., 2., 0.],
			&[0., 0., 2.],
		]);
		println!("{}", u.inverse());
		// [0.5, 0.0, 0.0]
		// [0.0, 0.5, 0.0]
		// [0.0, 0.0, 0.5]

	});

	let _ = panic::catch_unwind(|| {
		let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.], &[5., 7., 15.]]);
		println!("The determinant is {}", v.determinant());
		println!("{}\n", v.inverse());
	});

}
