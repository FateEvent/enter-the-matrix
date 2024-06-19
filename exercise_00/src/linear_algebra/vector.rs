use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>
}

impl fmt::Display for Vector<f32> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for n in self.values.iter() {
			write!(fmt, "[")?;
			write!(fmt, "{}", (n * 100.).round() / 100.)?;
			write!(fmt, "]")?;
			write!(fmt, "\n")?;
		}
		println!("The vector has {} rows.", self.get_rows());
		Ok(())
	}
}

impl Vector<f32> {
	pub fn new() -> Self {
		Vector {
			values: Vec::new()
		}
	}
	
	pub fn from(arr: &[f32]) -> Self {
		Vector {
			values: arr.to_vec()
		}
	}

	pub fn print(&self) {
		println!("{}", self);
	}

	pub fn get_rows(&self) -> usize {
		self.values.len()
	}

	fn vectors_have_equal_length(&self, other: Vector<f32>) -> bool {
		if self.get_rows() != other.get_rows() {
			panic!("Vectors must be of the same length.");
		};
		true
	}

	pub fn add(&mut self, other: Vector<f32>) {
		self.vectors_have_equal_length(other.clone());
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a += b.clone();
		}
	}

	pub fn sub(&mut self, other: Vector<f32>) {
		self.vectors_have_equal_length(other.clone());
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a -= b.clone();
		}
	}

	pub fn scl(&mut self, scalar: f32) {
		for el in self.values.iter_mut() {
			*el *= scalar.clone();
		}
	}
}
