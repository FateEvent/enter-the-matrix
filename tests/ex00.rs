use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;

#[test]
fn add_vectors() {
	let t = Vector::from([2., 3.]);
	let mut u = Vector::from([5., 7.]);

	u.add(t);
	assert_eq!(u, Vector::from([7., 10.]));
}

#[test]
fn subtract_vectors() {
	let mut t = Vector::from([7., 10.]);
	let u = Vector::from([5., 7.]);

	t.sub(u);
	assert_eq!(t, Vector::from([2., 3.]))
}

#[test]
fn scale_vectors() {
	let mut t = Vector::from([7., 10.]);

	t.scl(0.);
	assert_eq!(t, Vector::from([0., 0.]))
}

#[test]
fn scale_vectors_2() {
	let mut t = Vector::from([7., 10.]);

	t.scl(5.);
	assert_eq!(t, Vector::from([35., 50.]))
}

#[test]
fn add_matrices() {
	let m1 = Matrix::from([[5., 7.], [3., 4.]]);
	let mut m2 = Matrix::from([[5., 7.], [3., 4.]]);

	m2.add(m1);
	assert_eq!(m2, Matrix::from([[10., 14.], [6., 8.]]))
}

#[test]
fn subtract_matrices() {
	let m1 = Matrix::from([[5., 7.], [3., 4.]]);
	let mut m2 = Matrix::from([[5., 7.], [3., 4.]]);

	m2.sub(m1);
	assert_eq!(m2, Matrix::from([[0., 0.], [0., 0.]]))
}

#[test]
fn scale_matrices() {
	let mut m1 = Matrix::from([[14., 4.5], [35., 21.]]);

	m1.scl(5.);
	assert_eq!(m1, Matrix::from([[70., 22.5], [175., 105.]]))
}

#[test]
#[should_panic]
fn create_invalid_matrices() {
	let _m1 = Matrix::from(vec![vec![5., 7.], vec![3., 4.], vec![1.]]);
}

#[test]
#[should_panic]
fn subtract_invalid_matrices() {
	let mut m1 = Matrix::from([[5., 7.], [3., 4.]]);
	let m2 = Matrix::from([[5., 7.], [3., 4.], [1., 35.]]);

	m1.sub(m2);
}

#[test]
#[should_panic]
fn add_invalid_matrices() {
	let mut m1 = Matrix::from([[5., 7.], [3., 4.]]);
	let m2 = Matrix::from([[5., 7.], [3., 4.], [1., 35.]]);

	m1.add(m2);
}

#[test]
fn matrix_is_square() {

	let m1 = Matrix::from([[5., 7.,], [3., 4.]]);
	assert_eq!(m1.is_square(), true);
}

#[test]
fn matrix_is_square_2() {
	let m1 = Matrix::from([[5., 7.], [3., 4.], [1., 6.]]);
	assert_eq!(m1.is_square(), false);
}

#[test]
fn matrix_is_square_3() {
	let m1 = Matrix::from([[5., 7., 64.], [3., 4., 18.]]);
	assert_eq!(m1.is_square(), false);
}

#[test]
fn matrix_is_square_4() {
	let m1 = Matrix::from([[5., 7., 64.], [3., 4., 18.], [5., 22., 45.]]);
	assert_eq!(m1.is_square(), true);
}
