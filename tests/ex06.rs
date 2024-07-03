use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::cross_product;

#[test]
fn cross_product_example() {
    let u = Vector::from([4., 2., -3.]);
	let v = Vector::from([-2., -5., 16.]);

    assert_eq!(cross_product(u, v), Vector::from([17., -58., -16.]));
}

#[test]
fn cross_product_example_2() {
    let u = Vector::from([0., 0., 0.]);
    let v = Vector::from([1., 2., 3.]);

    assert_eq!(cross_product(u, v), Vector::from([0., 0., 0.]));
}

#[test]
fn cross_product_example_3() {
    let u = Vector::from([0., 0., 1.]);
	let v = Vector::from([1., 0., 0.]);

    assert_eq!(cross_product(u, v), Vector::from([0., 1., 0.]));
}
