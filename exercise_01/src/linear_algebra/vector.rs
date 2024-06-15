use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
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

	fn vectors_have_equal_length(&self, other: Vector<f32>) -> bool {
		if self.rows != other.rows {
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

	fn vec_arr_check_length(u: &[Vector<f32>]) -> bool {
		for v in u {
			if !Vector::vectors_have_equal_length(&u[0], v.clone()) {
				return false;
			}
		}
		true
	}

	pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
		Vector::vec_arr_check_length(u);
		let len = u[0].values.len();
		let mut combo: Vector<f32> = Vector::new();
		let mut sum: f32;

		for row in 0..len {
			sum = 0.0;
			for (v, coef) in u.iter().zip(coefs.iter()) {
				sum = (*coef).mul_add(v.values[row], sum);
			}
			combo.values.push(sum);
			combo.rows += 1;
		}
		return combo;
	}
}
