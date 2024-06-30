use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;
use matrix::linear_algebra::lerp;

#[test]
fn float_lerp() {
	assert_eq!(lerp(3., 4., 0.5), 3.5);
}

#[test]
fn vector_lerp() {
	assert_eq!(lerp(Vector::from(&[1., 1.]), Vector::from(&[2., 3.]), 0.8), Vector::from(&[1.8, 2.6000001]));
}

#[test]
fn vector_lerp_2() {
	assert_eq!(lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3), Vector::from(&[2.6, 1.3]));
}

#[test]
fn matrix_lerp() {
	assert_eq!(lerp(Matrix::from(&[&[2., 1.], &[3., 4.]]), Matrix::from(&[&[20., 10.], &[30., 40.]]), 0.5), Matrix::from(&[&[11., 5.5], &[16.5, 22.]]));
}
