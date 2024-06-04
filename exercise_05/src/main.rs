use std::panic;
use crate::linear_algebra::vector::Vector;
use crate::linear_algebra::angle_cos;

pub mod linear_algebra;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mCosine:\x1b[0m\n");
	let _ = panic::catch_unwind(|| {
		let u = Vector::from(&[0., 0., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from(&[1., 2., 3.]);
		println!("The norm of u is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(v.clone()));
		println!("Cos theta is: {}\n", angle_cos(u, v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from(&[1., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from(&[0., 1.]);
		println!("The norm of u is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(v.clone()));
		println!("Cos theta is: {}\n", angle_cos(u, v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from(&[1., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from(&[1., 0.]);
		println!("The norm of u is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(v.clone()));
		println!("Cos theta is: {}\n", angle_cos(u, v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from(&[-1., 1.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from(&[1., -1.]);
		println!("The norm of u is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(v.clone()));
		println!("Cos theta is: {}\n", angle_cos(u, v));
	});
}
