use std::panic;
use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;
use matrix::linear_algebra::lerp;
use matrix::linear_algebra::Complex;

fn main() {
	// let _ = panic::catch_unwind(|| {
	// 	println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
	// 	let mut t = Vector::from(&[2., 3.]);
	// 	t.print();
	// 	let mut u = Vector::from(&[5., 7.]);
	// 	u.print();

	// 	println!("\n\x1b[32mAddition:\x1b[0m\n");
	// 	u.add(t.clone());
	// 	u.print();

	// 	println!("\n\x1b[93mSubtraction:\x1b[0m\n");
	// 	u.sub(t.clone());
	// 	u.print();

	// 	println!("\n\x1b[34mScalar Multiplication:\x1b[0m\n");
	// 	t.scl(5.);
	// 	t.print();
	// });

	// let _ = panic::catch_unwind(|| {
	// 	println!("\n\x1b[31;1;4mo  xoX . -  Matrices  -  . Xox  o\x1b[0m\n");
	// 	let m1 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	// 	m1.print();
	// 	let mut m2 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	// 	m2.print();

	// 	println!("\n\x1b[32mAddition:\x1b[0m\n");
	// 	m2.add(m1.clone());
	// 	m2.print();

	// 	println!("\n\x1b[93mSubtraction:\x1b[0m\n");
	// 	m2.sub(m1.clone());
	// 	m2.print();

	// 	println!("\n\x1b[34mScalar Multiplication:\x1b[0m\n");
	// 	m2.scl(5.);
	// 	m2.print();
	// });

	// println!("\n\x1b[31;1;4mInvalid Matrices\x1b[0m\n");
	// let _ = panic::catch_unwind(|| {
	// 	let _m1 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1.]]);
	// });

	// let _ = panic::catch_unwind(|| {
	// 	let mut m1 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
	// 	let m2 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 35.]]);
	// 	m1.sub(m2.clone());
	// });

	let _ = panic::catch_unwind(|| {
		let complex_numbers = [
			Complex::new(1.0, 1.0),
			Complex::new(1.0, -1.0)
		];

		let vec: Vec<Complex<f32>> = complex_numbers.to_vec();

		let t = Vector::from_vec(vec);
		t.print();

		let v = Vector::from(&complex_numbers);

		println!("\n\x1b[31;1;4mThe Dot Product\x1b[0m\n");

		println!("{}", v.dot(t));

		println!("\n\x1b[31;1;4mThe Norms\x1b[0m\n");

		println!("{} {} {}", v.norm_1(), v.norm(), v.norm_inf());

		println!("\n\x1b[31;1;4mLinear Interpolation\x1b[0m\n");

		println!("{}", lerp(Complex::new(21., 0.),  Complex::new(42., 0.), 0.3));
		// 27.3

		let i = [
			Complex::new(2., 0.),
			Complex::new(1., 0.)
		];

		let j = [
			Complex::new(3., 0.),
			Complex::new(4., 0.)
		];

		let k = [
			Complex::new(20., 0.),
			Complex::new(10., 0.)
		];

		let l = [
			Complex::new(30., 0.),
			Complex::new(40., 0.)
		];
		
		println!("{}", lerp(Vector::from(&i), Vector::from(&[Complex::new(8., 0.), Complex::new(8., 0.)]), 0.8));
		println!("{}", lerp(Vector::from(&[Complex::new(15., 0.), Complex::new(21., 0.)]), Vector::from(&[Complex::new(13., 0.), Complex::new(31., 0.)]), 0.5));
		println!("{}", lerp(Matrix::from(&[&i, &j]), Matrix::from(&[&k, &l]), 0.5));

	});

	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4mRow Echelon Form\x1b[0m\n");

		let i = [
			Complex::new(2., 0.),
			Complex::new(1., 0.),
			Complex::new(4., 0.)
		];

		let j = [
			Complex::new(3., 0.),
			Complex::new(4., 0.),
			Complex::new(10., 0.)
		];

		let k = [
			Complex::new(20., 0.),
			Complex::new(10., 0.),
			Complex::new(1., 0.)
		];

		let l = [
			Complex::new(30., 0.),
			Complex::new(40., 0.)
		];

		let m = [
			Complex::new(2., 0.),
			Complex::new(1., 0.)
		];

		let n = [
			Complex::new(3., 15.),
			Complex::new(4., 2.)
		];

		let o = [
			Complex::new(20., 1.),
			Complex::new(10., 5.)
		];

		let p = [
			Complex::new(30., 40.),
			Complex::new(40., 4.)
		];

		let m1 = Matrix::from(&[&i, &j, &k]);
		m1.print();

		println!("{}", m1.reduced_row_echelon_form());

		let m2 = Matrix::from(&[&l, &m]);
		m2.print();

		println!("{}", m2.reduced_row_echelon_form());

		let m3 = Matrix::from(&[&n, &o, &p]);
		m3.print();

		println!("{}", m3.reduced_row_echelon_form());
	});


	// let _ = panic::catch_unwind(|| {
	// 	println!("\n\x1b[31;1;4mSquare Matrices\x1b[0m\n");
	// 	let m1 = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
	// 	m1.print();
	// 	println!("Is the matrix a square matrix? {}\n", m1.is_square());
	// 	let m2 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 6.]]);
	// 	m2.print();
	// 	println!("Is the matrix a square matrix? {}\n", m2.is_square());
	// 	let m3 = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
	// 	m3.print();
	// 	println!("Is the matrix a square matrix? {}\n", m3.is_square());
	// 	let m4 = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 22., 45.]]);
	// 	m4.print();
	// 	println!("Is the matrix a square matrix? {}\n", m4.is_square());
	// });

	// println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	// println!("\n\x1b[36mThe Rank of a Matrix:\x1b[0m\n");


	// let _ = panic::catch_unwind(|| {
	// 	let z: Matrix<f32> = Matrix::from(&[&[1., 0.,], &[0., 1.]]);
	// 	println!("{}", z.row_echelon_form());
	// 	println!("The rank is {}\n", z.rank());
	// 	// 2

	// 	let n = Matrix::from(&[
	// 		&[ 1., 2., 0., 0.],
	// 		&[ 2., 4., 0., 0.],
	// 		&[-1., 2., 1., 1.],
	// 	]);
	// 	println!("{}", n.row_echelon_form());
	// 	println!("The rank is {}\n", n.rank());
	// 	// 2

	// 	let u = Matrix::from(&[&[1., 2.,], &[2., 4.]]);
	// 	println!("{}", u.row_echelon_form());
	// 	println!("The rank is {}\n", u.rank());
	// 	// 1

	// 	let l: Matrix<f32> = Matrix::from(&[&[4., 2.,], &[7., 6.]]);
	// 	println!("{}", l.row_echelon_form());
	// 	println!("The rank is {}\n", l.rank());
	// 	// 2

	// 	let r = Matrix::from(&[&[5., 7., 15., 18.], &[3., 4., 5., 3.], &[5., 7., 15., 3.], &[1., 2., 1., 3.]]);
	// 	println!("{}", r.row_echelon_form());
	// 	println!("The rank is {}\n", r.rank());
	// 	// 4
	// 	let s = Matrix::from(&[&[5., 7., 15., 18., 17.], &[3., 4., 5., 3., 3.], &[5., 7., 15., 3., 7.], &[1., 2., 1., 3., 5.], &[5., 7., 15., 18., 7.]]);
	// 	println!("{}", s.row_echelon_form());
	// 	println!("The rank is {}\n", s.rank());
	// 	// 2

	// 	let t = Matrix::from(&[
	// 		&[ 8., 5., -2., 4.],
	// 		&[ 4., 2.5, 20., 4.],
	// 		&[ 8., 5., 1., 4.],
	// 		&[28., -4., 17., 1.],
	// 	]);
	// 	println!("{}", t.row_echelon_form());
	// 	println!("The rank is {}\n", t.rank());
	// 	// 4

	// 	let g = Matrix::from(&[
	// 		&[3., 2., 0.],
	// 		&[0., 0., 1.],
	// 		&[2., -2., 1.]
	// 	]);
	// 	println!("{}", g.row_echelon_form());
	// 	println!("The rank is {}\n", g.rank());
	// 	// 3

	// 	let h = Matrix::from(&[
	// 		&[8., 5., -2.],
	// 		&[4., 7., 20.],
	// 		&[7., 6., 1.],
	// 	]);
	// 	println!("{}", h.row_echelon_form());
	// 	println!("The rank is {}\n", h.rank());
	// 	// 3

	// 	let v = Matrix::from(&[&[5., 7., 15.], &[3., 4., 5.], &[5., 7., 15.]]);
	// 	println!("{}", v.row_echelon_form());
	// 	println!("The rank is {}\n", v.rank());
	// 	// 1

	// 	let m = Matrix::from(&[&[5.], &[3.], &[7.]]);
	// 	println!("{}", m.row_echelon_form());
	// 	println!("The rank is {}\n", m.rank());
	// 	// 1

	// 	let o = Matrix::from(&[
	// 		&[ 8., 5., -2.],
	// 		&[ 4., 7., 20.],
	// 		&[ 7., 6., 1.],
	// 		&[21., 18., 7.],
	// 	]);
	// 	println!("{}", o.row_echelon_form());
	// 	println!("The rank is {}\n", o.rank());
	// 	// 3
	// });
}
