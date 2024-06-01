use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

impl fmt::Display for Vector<f32> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		println!("The vector has {} rows.", self.rows);
		println!("The magnitude or length of the vector is {}.", self.magnitude());
		write!(fmt, "[")?;
		for (i, n) in self.values.iter().enumerate() {
			write!(fmt, "{}", n)?;
			if i < self.values.len() - 1 {
				write!(fmt, " ")?;
			}
		}
		write!(fmt, "]")?;
		Ok(())
	}
}

impl Vector<f32> {
	pub fn new() -> Self {
		Vector {
			values: Vec::new(),
			rows: 0
		}
	}
	
	pub fn from(arr: &[f32]) -> Self {
		Vector {
			values: arr.to_vec(),
			rows: arr.len()
		}
	}

	pub fn print(&self) {
		println!("{}", self);
	}

	pub fn get_rows(&self) -> usize {
		self.rows
	}

	pub fn magnitude(&self) -> f32 {
		let mut sum: f32 = 0.0;
		for el in self.values.iter() {
			sum += el.clone() * el.clone();
		}
		return sum.sqrt();
	}

	fn vectors_have_equal_length(&self, other: Vector<f32>) -> bool {
		if self.rows != other.rows {
			panic!("Vectors must be of the same length.");
		};
		true
	}

	pub fn add(&mut self, other: Vector<f32>) {
		if self.vectors_have_equal_length(other.clone()) {
			for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
				*a += b.clone();
			}
		}
	}

	pub fn sub(&mut self, other: Vector<f32>) {
		if self.vectors_have_equal_length(other.clone()) {
			for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
				*a -= b.clone();
			}
		}
	}

	pub fn scl(&mut self, scalar: f32) {
		for el in self.values.iter_mut() {
			*el *= scalar.clone();
		}
	}
}
