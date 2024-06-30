use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::angle_cos;

#[test]
fn angle_cos_example() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);

    assert_eq!(angle_cos(u, v), 0.);
}

#[test]
fn angle_cos_example_2() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);

    assert_eq!(angle_cos(u, v), 1.);
}

#[test]
fn angle_cos_example_3() {
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);

    assert_eq!(angle_cos(u, v), 1.);
}
