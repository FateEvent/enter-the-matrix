use std::panic;
use matrix::linear_algebra::vector::Vector;

fn main() {
	println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
	println!("\n\x1b[32mCosine:\x1b[0m\n");
	let _ = panic::catch_unwind(|| {
		let u = Vector::from([0., 0., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from([1., 2., 3.]);
		println!("The norm of v is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(&v));
		println!("Cos theta is: {}\n", u.angle_cos(&v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([1., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from([0., 1.]);
		println!("The norm of v is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(&v));
		println!("Cos theta is: {}\n", u.angle_cos(&v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([1., 0.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from([1., 0.]);
		println!("The norm of v is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(&v));
		println!("Cos theta is: {}\n", u.angle_cos(&v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([-1., 1.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from([1., -1.]);
		println!("The norm of v is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(&v));
		println!("Cos theta is: {}\n", u.angle_cos(&v));
	});

	let _ = panic::catch_unwind(|| {
		let u = Vector::from([-2., 4.]);
		println!("The norm of u is: {}", u.norm());
		let v = Vector::from([-4., 8.]);
		println!("The norm of v is: {}", v.norm());
		println!("The dot product of u and v is: {}", u.dot(&v));
		println!("Cos theta is: {}\n", u.angle_cos(&v));
	});
}
