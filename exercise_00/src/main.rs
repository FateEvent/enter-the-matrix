use std::panic;
use crate::linear_algebra::Vector;
use crate::linear_algebra::Matrix;

pub mod linear_algebra;
 
fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		let mut t = Vector::from(&[2., 3.]);
		t.print();
		let mut u = Vector::from(&[5., 7.]);
		u.print();

		println!("\n\x1b[32mAddition:\x1b[0m\n");
		u.add(t.clone());
		u.print();

		println!("\n\x1b[93mSubtraction:\x1b[0m\n");
		u.sub(t.clone());
		u.print();

		println!("\n\x1b[34mScalar multiplication:\x1b[0m\n");
		t.scl(5.);
		t.print();
	});

	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4mo  xoX . -  Matrices  -  . Xox  o\x1b[0m\n");
		let m1 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
		m1.print();
		let mut m2 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
		m2.print();

		println!("\n\x1b[32mAddition:\x1b[0m\n");
		m2.add(m1.clone());
		m2.print();

		println!("\n\x1b[93mSubtraction:\x1b[0m\n");
		m2.sub(m1.clone());
		m2.print();

		println!("\n\x1b[34mScalar multiplication:\x1b[0m\n");
		m2.scl(5.);
		m2.print();
	});

	println!("\n\x1b[31;1;4mInvalid matrices\x1b[0m\n");
	let _ = panic::catch_unwind(|| {
		let _m1 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1.]]);
	});

	let _ = panic::catch_unwind(|| {
		let mut m1 = Matrix::from(&[&[5., 7.], &[3., 4.]]);
		let m2 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 35.]]);
		m1.sub(m2.clone());
	});

	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4mSquare matrices\x1b[0m\n");
		let m1 = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
		m1.print();
		println!("Is the matrix a square matrix? {}\n", m1.is_square());
		let m2 = Matrix::from(&[&[5., 7.], &[3., 4.], &[1., 6.]]);
		m2.print();
		println!("Is the matrix a square matrix? {}\n", m2.is_square());
		let m3 = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.]]);
		m3.print();
		println!("Is the matrix a square matrix? {}\n", m3.is_square());
		let m4 = Matrix::from(&[&[5., 7., 64.], &[3., 4., 18.], &[5., 22., 45.]]);
		m4.print();
		println!("Is the matrix a square matrix? {}\n", m4.is_square());
	});
}
