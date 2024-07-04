use matrix::linear_algebra::vector::Vector;

#[test]
fn linear_combination() {
	let t = Vector::from([2., 3.]);
	let u = Vector::from([5., 7.]);

	let v = Vector::<f32>::linear_combination(&[t, u], &[4., 7.]);
	assert_eq!(v, Vector::from([43., 61.]));
}
