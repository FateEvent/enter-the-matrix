use std::panic;
use crate::vector::Vector;

pub mod vector;
pub mod matrix;

fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		let t = Vector::from(&[2., 3.]);
		t.print();
		let u = Vector::from(&[5., 7.]);
		u.print();

		println!("\n\x1b[32mLinear combination:\x1b[0m\n");
		let v = Vector::linear_combination(&[t, u], &[4., 7.]);
		v.print();
	});
}
