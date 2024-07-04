use std::panic;
use matrix::linear_algebra::vector::Vector;

fn main() {
	let _ = panic::catch_unwind(|| {
		println!("\n\x1b[31;1;4m   o  .  -  Vectors  -  .  o\x1b[0m\n");
		
		println!("\n\x1b[32mDot Product:\x1b[0m\n");
		let a = Vector::from([1., 3.]);
		a.print();
		let b = Vector::from([-5., 2.]);
		b.print();
		println!("{}", a.dot(&b));
		let t = Vector::from([2., 3.]);
		t.print();
		let u = Vector::from([5., 7.]);
		u.print();
		println!("{}", t.dot(&u));
		t.print();
		let v = Vector::linear_combination(&[t.clone(), u], &[4., 7.]);
		v.print();
		println!("{}", t.dot(&v));
		let p = Vector::from([4., -1., 2.]);
		p.print();
		let q = Vector::from([1., 2., 4.]);
		q.print();
		println!("{}", p.dot(&q));
	});
}
