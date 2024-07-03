use matrix::linear_algebra::vector::Vector;

#[test]
fn angle_cos_example() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);

    assert_eq!(u.angle_cos(&v), 0.);
}

#[test]
fn angle_cos_example_2() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);

    assert_eq!(u.angle_cos(&v), 1.);
}

#[test]
fn angle_cos_example_3() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);

    assert_eq!(u.angle_cos(&v), 1.);
}
