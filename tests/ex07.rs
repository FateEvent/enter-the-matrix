use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::matrix::Matrix;

#[test]
fn matrix_multiplication_example() {
    let u = Matrix::from(&[&[5., 7.,], &[3., 4.]]);
	let v = Matrix::from(&[&[5., 7.], &[3., 4.]]);

    assert_eq!(u.mul_mat(v), Matrix::from(&[&[46., 63.], &[27., 37.]]));
}

#[test]
fn matrix_multiplication_example_2() {
    let u = Matrix::from(&[&[1., 3.], &[2., 4.], &[2., 5.]]);
	let v = Matrix::from(&[&[1., 3., 2., 2.], &[2., 4., 5., 1.]]);

    assert_eq!(u.mul_mat(v), Matrix::from(&[
		&[7., 15., 17., 5.],
		&[10., 22., 24., 8.],
		&[12., 26., 29., 9.]
	]));
}

#[test]
fn matrix_vector_multiplication_example() {
    let u = Matrix::from(&[&[4., 3., 1.], &[6., 7., 2.]]);
	let v = Vector::from(&[3., 1., 4.]);

    assert_eq!(u.mul_vec(v), Vector::from(&[19., 33.]));
}

#[test]
#[should_panic]
fn invalid_matrix_vector_multiplication_example() {
    let u = Matrix::from(&[&[4., 3., 1.], &[6., 7., 2.]]);
	let v = Vector::from(&[3., 1., 4., 5.]);

	u.mul_vec(v);
}
