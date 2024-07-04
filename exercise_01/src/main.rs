use std::panic;
use matrix::linear_algebra::vector::Vector;

fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		let t = Vector::from([2., 3.]);
		t.print();
		let u = Vector::from([5., 7.]);
		u.print();

		println!("\n\x1b[32mLinear Combination:\x1b[0m\n");
		println!("{}", Vector::linear_combination(&[t, u], &[4., 7.]));
	});
}
