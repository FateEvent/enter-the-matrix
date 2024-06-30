use matrix::linear_algebra::vector::Vector;

#[test]
fn norm_1_example() {
    let u = Vector::from(&[0., 0., 0.]);

    assert_eq!(u.norm_1(), 0.);
}

#[test]
fn norm_example() {
    let u = Vector::from(&[0., 0., 0.]);

    assert_eq!(u.norm(), 0.);
}

#[test]
fn norm_inf_example() {
    let u = Vector::from(&[0., 0., 0.]);

    assert_eq!(u.norm_inf(), 0.);
}

#[test]
fn norm_1_example_2() {
    let u = Vector::from(&[-1., -2.]);

    assert_eq!(u.norm_1(), 3.);
}

#[test]
fn norm_example_2() {
    let u = Vector::from(&[-1., -2.]);

    assert_eq!(u.norm(), 2.236068);
}

#[test]
fn norm_inf_example_2() {
    let u = Vector::from(&[-1., -2.]);

    assert_eq!(u.norm_inf(), 2.);
}
