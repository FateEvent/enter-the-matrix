use std::panic;
use matrix::linear_algebra::matrix::Matrix;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[36mThe Rank of a Matrix:\x1b[0m\n");


	let _ = panic::catch_unwind(|| {
		let z: Matrix<f32> = Matrix::from([[1., 0.,], [0., 1.]]);
		println!("{}", z.row_echelon_form());
		println!("The rank is {}\n", z.rank());
		// 2

		let n = Matrix::from([
			[ 1., 2., 0., 0.],
			[ 2., 4., 0., 0.],
			[-1., 2., 1., 1.],
		]);
		println!("{}", n.row_echelon_form());
		println!("The rank is {}\n", n.rank());
		// 2

		let u = Matrix::from([[1., 2.,], [2., 4.]]);
		println!("{}", u.row_echelon_form());
		println!("The rank is {}\n", u.rank());
		// 1

		let l: Matrix<f32> = Matrix::from([[4., 2.,], [7., 6.]]);
		println!("{}", l.row_echelon_form());
		println!("The rank is {}\n", l.rank());
		// 2

		let r = Matrix::from([[5., 7., 15., 18.], [3., 4., 5., 3.], [5., 7., 15., 3.], [1., 2., 1., 3.]]);
		println!("{}", r.row_echelon_form());
		println!("The rank is {}\n", r.rank());
		// 4
		let s = Matrix::from([[5., 7., 15., 18., 17.], [3., 4., 5., 3., 3.], [5., 7., 15., 3., 7.], [1., 2., 1., 3., 5.], [5., 7., 15., 18., 7.]]);
		println!("{}", s.row_echelon_form());
		println!("The rank is {}\n", s.rank());
		// 2

		let t = Matrix::from([
			[ 8., 5., -2., 4.],
			[ 4., 2.5, 20., 4.],
			[ 8., 5., 1., 4.],
			[28., -4., 17., 1.],
		]);
		println!("{}", t.row_echelon_form());
		println!("The rank is {}\n", t.rank());
		// 4

		let g = Matrix::from([
			[3., 2., 0.],
			[0., 0., 1.],
			[2., -2., 1.]
		]);
		println!("{}", g.row_echelon_form());
		println!("The rank is {}\n", g.rank());
		// 3

		let h = Matrix::from([
			[8., 5., -2.],
			[4., 7., 20.],
			[7., 6., 1.],
		]);
		println!("{}", h.row_echelon_form());
		println!("The rank is {}\n", h.rank());
		// 3

		let v = Matrix::from([[5., 7., 15.], [3., 4., 5.], [5., 7., 15.]]);
		println!("{}", v.row_echelon_form());
		println!("The rank is {}\n", v.rank());
		// 1

		let m = Matrix::from([[5.], [3.], [7.]]);
		println!("{}", m.row_echelon_form());
		println!("The rank is {}\n", m.rank());
		// 1

		let o = Matrix::from([
			[ 8., 5., -2.],
			[ 4., 7., 20.],
			[ 7., 6., 1.],
			[21., 18., 7.],
		]);
		println!("{}", o.row_echelon_form());
		println!("The rank is {}\n", o.rank());
		// 3
	});

}
