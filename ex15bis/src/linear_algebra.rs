pub use core::fmt;
pub use core::ops::Neg;

pub use num::complex::Complex;
pub use num::Num;
pub use num::Zero;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

impl<K> Vector<K>
where Vector<K>: std::fmt::Display,
K: Copy + Clone + num::Num + std::ops::AddAssign
+ std::ops::SubAssign + std::ops::MulAssign + std::fmt::Display
+ std::ops::Neg<Output = K> {
	pub fn new() -> Self {
		Vector {
			values: Vec::new(),
			rows: 0
		}
	}
	
	pub fn from(arr: &[K]) -> Self {
		Vector {
			values: arr.to_vec(),
			rows: arr.len()
		}
	}

	pub fn from_vec(vec: Vec<K>) -> Self {
		Vector {
			values: vec.clone(),
			rows: vec.len()
		}
	}

	pub fn print(&self) {
		println!("{}", self);
	}
}

impl std::ops::Mul<Vector<f32>> for f32 {
	type Output = Vector<f32>;

	fn mul(self, _rhs: Vector<f32>) -> Vector<f32> {
		let mut a: Vector<f32> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(*el * self);
			a.rows += 1;
		}
		a
	}
}

impl fmt::Display for Vector<f32> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for n in self.values.iter() {
			write!(fmt, "[")?;
			write!(fmt, "{}", (n * 100.).round() / 100.)?;
			write!(fmt, "]")?;
			write!(fmt, "\n")?;
		}
		println!("The vector has {} rows.", self.rows);
		Ok(())
	}
}

impl fmt::Display for Vector<Complex<f32>> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for n in self.values.iter() {
			write!(fmt, "[")?;
			write!(fmt, "{}", n)?;
			write!(fmt, "]")?;
			write!(fmt, "\n")?;
		}
		println!("The vector has {} rows.", self.rows);
		Ok(())
	}
}

impl std::ops::Mul<Vector<Complex<f32>>> for Complex<f32> {
	type Output = Vector<Complex<f32>>;

	fn mul(self, _rhs: Vector<Complex<f32>>) -> Vector<Complex<f32>> {
		let mut a: Vector<Complex<f32>> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(*el * self);
			a.rows += 1;
		}
		a
	}
}

impl Vector<Complex<f32>> {
	pub fn norm(&self) -> Complex<f32> {
		let mut sum = Complex::<f32>::zero();
		for el in self.values.iter() {
			sum += el.powf(2.);
		}
		return sum.powf(0.5);
	}
}