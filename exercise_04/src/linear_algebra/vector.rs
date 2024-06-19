use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>
}

impl std::ops::Sub<Vector<f32>> for Vector<f32> {
	type Output = Vector<f32>;

	fn sub(self, _rhs: Vector<f32>) -> Vector<f32> {
		self.vectors_have_equal_length(_rhs.clone());
		let mut diff: Vector<f32> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			diff.values.push(a - b);
		}
		diff
	}
}

impl std::ops::Add<Vector<f32>> for Vector<f32> {
	type Output = Vector<f32>;

	fn add(self, _rhs: Vector<f32>) -> Vector<f32> {
		self.vectors_have_equal_length(_rhs.clone());
		let mut sum: Vector<f32> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			sum.values.push(a + b);
		}
		sum
	}
}

impl std::ops::Mul<Vector<f32>> for f32 {
	type Output = Vector<f32>;

	fn mul(self, _rhs: Vector<f32>) -> Vector<f32> {
		let mut a: Vector<f32> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(*el * self);
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
		println!("The vector has {} rows.", self.rows());
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

	pub fn from_vec(vec: Vec<f32>) -> Self {
		Vector {
			values: vec
		}
	}

	pub fn print(&self) {
		println!("{}", self);
	}

	pub fn rows(&self) -> usize {
		self.values.len()
	}

	fn vectors_have_equal_length(&self, other: Vector<f32>) -> bool {
		if self.rows() != other.rows() {
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

	pub fn mul_add(&self, a: f32, b: &Vector<f32>) -> Vector<f32> {
		self.vectors_have_equal_length(b.clone());
		return Vector::from_vec(self.values.iter().zip(b.values.iter())
		.map(|(u, v)| u.mul_add(a, *v)).collect());
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
		}
		return combo;
	}

	pub fn dot(&self, v: Vector<f32>) -> f32 {
		self.vectors_have_equal_length(v.clone());
		let mut sum: f32 = 0.0;
		for (e1, e2) in self.values.iter().zip(v.values.iter()) {
			sum += e1 * e2;
		}
		sum		
	}

	pub fn norm_1(&self) -> f32 {
		let mut sum: f32 = 0.0;
		for el in self.values.iter() {
			sum += el.abs();
		}
		return sum;
	}

	pub fn norm(&self) -> f32 {
		let mut sum: f32 = 0.0;
		for el in self.values.iter() {
			sum += el.powf(2.);
		}
		return sum.powf(0.5);
	}

	pub fn norm_inf(&self) -> f32 {
		let mut max: f32 = 0.0;
		for el in self.values.iter() {
			if max < el.abs() { max = el.abs(); }
		}
		return max;
	}
}
