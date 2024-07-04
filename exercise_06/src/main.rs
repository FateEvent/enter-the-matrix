use std::panic;
use matrix::linear_algebra::vector::Vector;
use matrix::linear_algebra::cross_product;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mCross Product:\x1b[0m\n");
	let _ = panic::catch_unwind(|| {
		let u = Vector::from([0., 0., 0.]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from([1., 2., 3.]);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", cross_product(&u, &v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([0., 0., 1.]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from([1., 0., 0.]);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", cross_product(&u, &v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([1., 2., 3.]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from([4., 5., 6.]);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", cross_product(&u, &v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([4., 2., -3.]);
		println!("u is equal to:\n{}", u);
		let v = Vector::from([-2., -5., 16.]);
		println!("v is equal to:\n{}", v);
		println!("The cross product of u × v is:\n{}\n", cross_product(&u, &v));
	});
}
