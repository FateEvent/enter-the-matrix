use std::panic;
use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;
use matrix::linear_algebra::Complex;
use matrix::linear_algebra::lerp;

fn main() {

	let _ = panic::catch_unwind(|| {
		let i = [
			Complex::new(2., 0.),
			Complex::new(1., 0.)
		];

		let j = [
			Complex::new(3., 0.),
			Complex::new(4., 0.)
		];

		println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");

		let mut t = Vector::from(i);
		t.print();
		let mut u = Vector::from(j);
		u.print();

		println!("\n\x1b[32mAddition:\x1b[0m\n");
		u.add(t.clone());
		u.print();

		println!("\n\x1b[93mSubtraction:\x1b[0m\n");
		u.sub(t.clone());
		u.print();

		println!("\n\x1b[34mScalar Multiplication:\x1b[0m\n");
		t.scl(Complex::from(5.));
		t.print();
	});

	let _ = panic::catch_unwind(|| {
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

		println!("\n\x1b[31;1;4mo  xoX . -  Matrices  -  . Xox  o\x1b[0m\n");
		let m1: Matrix<Complex<f32>> = Matrix::from([k, l]);
		m1.print();
		let mut m2: Matrix<Complex<f32>> = Matrix::from([i, j]);
		m2.print();

		println!("\n\x1b[32mAddition:\x1b[0m\n");
		m2.add(m1.clone());
		m2.print();

		println!("\n\x1b[93mSubtraction:\x1b[0m\n");
		m2.sub(m1.clone());
		m2.print();

		println!("\n\x1b[34mScalar Multiplication:\x1b[0m\n");
		m2.scl(Complex::from(5.));
		m2.print();
	});

	let _ = panic::catch_unwind(|| {

		let complex_numbers = [
			Complex::new(1.0, 1.0),
			Complex::new(1.0, -1.0)
		];

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

		let m = [
			Complex::new(2., 0.),
			Complex::new(3., 0.)
		];

		let n = [
			Complex::new(5., 0.),
			Complex::new(7., 0.)
		];

		let vec: Vec<Complex<f32>> = complex_numbers.to_vec();

		let t = Vector::from_vec(vec);
		let v = Vector::from(complex_numbers);

		println!("\n\x1b[31;1;4mLinear Combination\x1b[0m\n");

		let u = Vector::from(m);
		let v1 = Vector::from(n);

		println!("{}", Vector::linear_combination(&[u, v1], &[4., 7.]));

		let uwu = Vector::from(i);
		let v2 = Vector::from(j);

		println!("{}", Vector::linear_combination(&[uwu, v2], &[2., 3.5.]));

		println!("\n\x1b[31;1;4mLinear Interpolation\x1b[0m\n");

		println!("{}", lerp(Complex::new(21., 0.),  Complex::new(42., 0.), 0.3));
		// 27.3
		
		println!("{}", lerp(Vector::from(i), Vector::from([Complex::new(8., 0.), Complex::new(8., 0.)]), 0.8));
		println!("{}", lerp(Vector::from([Complex::new(15., 0.), Complex::new(21., 0.)]), Vector::from([Complex::new(13., 0.), Complex::new(31., 0.)]), 0.5));
		println!("{}", lerp(Matrix::from([i, j]), Matrix::from([k, l]), 0.5));

		println!("\n\x1b[31;1;4mDot Product\x1b[0m\n");

		println!("{}", v.dot(t));

		println!("\n\x1b[31;1;4mNorms\x1b[0m\n");

		println!("{} {} {}", v.norm_1(), v.norm(), v.norm_inf());
	});

	let _ = panic::catch_unwind(|| {
		let q = [
			Complex::new(2., 0.),
			Complex::new(1., 0.),
			Complex::new(4., 0.)
		];

		let r = [
			Complex::new(3., 0.),
			Complex::new(4., 0.),
			Complex::new(10., 0.)
		];

		let s = [
			Complex::new(20., 0.),
			Complex::new(10., 0.),
			Complex::new(1., 0.)
		];

		println!("\n\x1b[31;1;4mRow Echelon Form\x1b[0m\n");

		let m1 = Matrix::from([s, q, r]);
		println!("{}\n", m1);

		println!("\n{}", m1.row_echelon_form());
		println!("\n{}", m1.reduced_row_echelon_form());

		let m2 = Matrix::from([s, q]);
		println!("{}\n", m2);

		println!("\n{}", m2.row_echelon_form());
		println!("\n{}", m2.reduced_row_echelon_form());

		let m3 = Matrix::from([q, s, r]);
		println!("{}\n", m3);

		println!("\n{}", m3.row_echelon_form());
		println!("\n{}", m3.reduced_row_echelon_form());
	});

	let _ = panic::catch_unwind(|| {
		// unit vectors
		let a = [
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let b = [
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];

		// three-dimensional vectors
		let q = [
			Complex::new(2., 0.),
			Complex::new(1., 0.),
			Complex::new(4., 0.)
		];

		let r = [
			Complex::new(3., 0.),
			Complex::new(4., 0.),
			Complex::new(10., 0.)
		];

		let s = [
			Complex::new(20., 0.),
			Complex::new(10., 0.),
			Complex::new(1., 0.)
		];

		println!("\n\x1b[31;1;4mSquare Matrices\x1b[0m\n");
		let m1 = Matrix::from([a, b]);
		m1.print();
		println!("Is the matrix a square matrix? {}\n", m1.is_square());
		let m2 = Matrix::from([s, q, r]);
		m2.print();
		println!("Is the matrix a square matrix? {}\n", m2.is_square());
		let m3 = Matrix::from([r, s]);
		m3.print();
		println!("Is the matrix a square matrix? {}\n", m3.is_square());
	});

	println!("\n\x1b[31;1;4m   o  .  -  Matrices  -  .  o\x1b[0m\n");

	println!("\n\x1b[36mThe Rank of a Matrix:\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		// unit vectors
		let a = [
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let b = [
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];

		let c = [
			Complex::new(1., 0.),
			Complex::new(0., 0.),
			Complex::new(0., 0.)
		];

		let d = [
			Complex::new(0., 0.),
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let e = [
			Complex::new(0., 0.),
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];

		// more complex values
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

		let z: Matrix<Complex<f32>> = Matrix::from([a, b]);
		println!("{}", z.row_echelon_form());
		println!("The rank is {}\n", z.rank());

		let m1: Matrix<Complex<f32>> = Matrix::from([c, d, e]);
		println!("{}", m1.row_echelon_form());
		println!("The rank is {}\n", m1.rank());

		let u: Matrix<Complex<f32>> = Matrix::from([l, m]);
		println!("{}", u.row_echelon_form());
		println!("The rank is {}\n", u.rank());

		let t: Matrix<Complex<f32>> = Matrix::from([n, o]);
		println!("{}", t.row_echelon_form());
		println!("The rank is {}\n", t.rank());

		// real numbers
		let q1 = [2., 1., 4.];
	
		let r1 = [3., 4., 10.];
	
		let s1 = [20., 10., 1.];

		println!("Comparing with real numbers:\n");
		let m4: Matrix<f32> = Matrix::from([q1, s1, r1]);
		m4.print();

		println!("\n{}", m4.row_echelon_form());
		println!("The rank is {}\n", t.rank());

		println!("\n{}", m4.reduced_row_echelon_form());
		println!("The rank is {}\n", t.rank());
	});

	let _ = panic::catch_unwind(|| {
		// unit vectors
		let c = [
			Complex::new(1., 0.),
			Complex::new(0., 0.),
			Complex::new(0., 0.)
		];

		let d = [
			Complex::new(0., 0.),
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let e = [
			Complex::new(0., 0.),
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];
	});

	println!("\n\x1b[34;4mAngle Cosine\x1b[0m\n");

	let _ = panic::catch_unwind(|| {
		// unit vectors
		let a = [
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let b = [
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];

		// more complex vectors
		let c = [
			Complex::new(-1., 0.),
			Complex::new(1., 0.)
		];

		let d = [
			Complex::new(1., 0.),
			Complex::new(-1., 0.)
		];

		let e = [
			Complex::new(-2., 0.),
			Complex::new(4., 0.)
		];

		let f = [
			Complex::new(-4., 0.),
			Complex::new(8., 0.)
		];

		let u = Vector::from(a);
		let v = Vector::from(b);
		println!("Cos theta is: {}\n", u.angle_cos(&v));

		let u2 = Vector::from(a);
		let v2 = Vector::from(a);
		println!("Cos theta is: {}\n", u2.angle_cos(&v2));

		let u3 = Vector::from(c);
		let v3 = Vector::from(d);
		println!("Cos theta is: {}\n", u3.angle_cos(&v3));

		let u4 = Vector::from(e);
		let v4 = Vector::from(f);
		println!("Cos theta is: {}\n", u4.angle_cos(&v4));
	});

	let _ = panic::catch_unwind(|| {
		// unit vectors
		let a = [
			Complex::new(1., 0.),
			Complex::new(0., 0.),
			Complex::new(0., 0.)
		];

		let b = [
			Complex::new(0., 0.),
			Complex::new(1., 0.),
			Complex::new(0., 0.)
		];

		let c = [
			Complex::new(0., 0.),
			Complex::new(0., 0.),
			Complex::new(1., 0.)
		];

		// more complex vectors
		let d = [
			Complex::new(1., 0.),
			Complex::new(2., 0.),
			Complex::new(3., 0.)
		];

		let e = [
			Complex::new(4., 0.),
			Complex::new(5., 0.),
			Complex::new(6., 0.)
		];

		let u = Vector::from(b);
		let v = Vector::from(a);
		println!("The cross product of u × v is:\n{}\n", cross_product(u, v));

		let u2 = Vector::from(d);
		println!("u is equal to:\n{}", u);
		let v2 = Vector::from(e);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", cross_product(u2, v2));
	});
}
