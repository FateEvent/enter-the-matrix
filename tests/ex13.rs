use matrix::linear_algebra::matrix::Matrix;

#[test]
fn matrix_rank() {
	let m = Matrix::from([[1., 0.,], [0., 1.]]);

	assert_eq!(m.rank(), 2);
}

#[test]
fn matrix_rank_2() {
	let m = Matrix::from([
		[ 1., 2., 0., 0.],
		[ 2., 4., 0., 0.],
		[-1., 2., 1., 1.],
	]);

	assert_eq!(m.rank(), 2);
}

#[test]
fn matrix_rank_3() {
	let m = Matrix::from([
		[5., 7., 15., 18.],
		[3., 4., 5., 3.],
		[5., 7., 15., 3.],
		[1., 2., 1., 3.]
	]);

	assert_eq!(m.rank(), 4);
}

#[test]
fn matrix_rank_4() {
	let m = Matrix::from([
		[8., 5., -2.],
		[4., 7., 20.],
		[7., 6., 1.],
	]);

	assert_eq!(m.rank(), 3);
}

#[test]
fn matrix_rank_5() {
	let m = Matrix::from([[5.], [3.], [7.]]);

	assert_eq!(m.rank(), 1);
}
