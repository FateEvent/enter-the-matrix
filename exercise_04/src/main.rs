use std::panic;
use crate::linear_algebra::vector::Vector;

pub mod linear_algebra;

fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
		println!("\n\x1b[32mNorm:\x1b[0m\n");
		let u = Vector::from(&[0., 0., 0.]);
		println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
		// 0.0, 0.0, 0.0
		let v = Vector::from(&[1., 2., 3.]);
		println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
		// 6.0, 3.74165738, 3.0
		let w = Vector::from(&[-1., -2.]);
		println!("{}, {}, {}", w.norm_1(), w.norm(), w.norm_inf());
		// 3.0, 2.236067977, 2.0
		let p = Vector::from(&[4., -1., 2.]);
		p.print();
		println!("{}, {}, {}", p.norm_1(), p.norm(), p.norm_inf());
		let q = Vector::from(&[1., 2., 4.]);
		q.print();
		println!("{}, {}, {}", q.norm_1(), q.norm(), q.norm_inf());
	});
}
