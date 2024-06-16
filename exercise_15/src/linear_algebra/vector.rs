use core::fmt;

use crate::linear_algebra::Matrix;
use crate::linear_algebra::Complex;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

impl<K: Default + Copy + Clone + std::ops::AddAssign + std::ops::Add + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::Sub + std::ops::Sub<Output = K> + std::ops::MulAssign
+ std::ops::Mul<Output = K>> std::ops::Sub<Vector<K>> for Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display {
	type Output = Vector<K>;

	fn sub(self, _rhs: Vector<K>) -> Vector<K> {
		self.vectors_have_equal_length(_rhs.clone());
		let mut diff: Vector<K> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			diff.values.push(*a - *b);
			diff.rows += 1;
		}
		diff
	}
}

impl<K: Default + Copy + Clone + std::ops::AddAssign + std::ops::Add + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::Sub + std::ops::Sub<Output = K> + std::ops::MulAssign
+ std::ops::Mul<Output = K>> std::ops::Add<Vector<K>> for Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display {
	type Output = Vector<K>;

	fn add(self, _rhs: Vector<K>) -> Vector<K> {
		self.vectors_have_equal_length(_rhs.clone());
		let mut sum: Vector<K> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			sum.values.push(*a + *b);
			sum.rows += 1;
		}
		sum
	}
}

impl<K: Default + Copy + Clone + std::ops::AddAssign + std::ops::Add + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::Sub + std::ops::Sub<Output = K> + std::ops::MulAssign
+ std::ops::Mul<Output = K>> std::ops::Index<usize> for Vector<K> {
	type Output = K;

	fn index(&self, index: usize) -> &K {
		&self.values[index]
	}
}

impl<K: Default + Copy + Clone + std::ops::AddAssign + std::ops::Add + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::Sub + std::ops::Sub<Output = K> + std::ops::MulAssign
+ std::ops::Mul<Output = K>> Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display {
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

	pub fn get_rows(&self) -> usize {
		self.rows
	}

	pub fn get_values(&self) -> Vec<K> {
		self.values.clone()
	}

	pub fn push(&mut self, val: K) {
		self.values.push(val);
		self.rows += 1;
	}

	fn vectors_have_equal_length(&self, other: Vector<K>) -> bool {
		if self.rows != other.rows {
			panic!("Vectors must be of the same length.");
		};
		true
	}

	pub fn add(&mut self, other: Vector<K>) {
		self.vectors_have_equal_length(other.clone());
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a += b.clone();
		}
	}

	pub fn sub(&mut self, other: Vector<K>) {
		self.vectors_have_equal_length(other.clone());
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a -= b.clone();
		}
	}

	pub fn scl(&mut self, scalar: K) {
		for el in self.values.iter_mut() {
			*el *= scalar.clone();
		}
	}

	fn vec_arr_check_length(u: &[Vector<K>]) -> bool {
		for v in u {
			if !Vector::vectors_have_equal_length(&u[0], v.clone()) {
				return false;
			}
		}
		true
	}

	pub fn capture_column(matrix: Matrix<K>, index: usize) -> Vector<K> {
		if index >= matrix.shape().1 {
			panic!("Column index out of bounds.")
		}
		let mut capture = Vector::new();

		for v in matrix.get_values().iter() {
			capture.values.push(v[index]);
		}
		capture
	}

	pub fn capture_row(matrix: Matrix<K>, index: usize) -> Vector<K> {
		if index >= matrix.shape().0 {
			panic!("Row index out of bounds.")
		}
		let capture = Vector::from_vec(matrix.get_values()[index].clone());

		capture
	}

	// pub fn dot(&self, v: Vector<K>) -> K {
	// 	self.vectors_have_equal_length(v.clone());
	// 	let mut sum = K::default();
	// 	for (e1, e2) in self.values.iter().zip(v.values.iter()) {
	// 		sum += *e1 * *e2;
	// 	}
	// 	sum		
	// }
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

impl Vector<f32> {
	pub fn mul_add(&self, a: f32, b: &Vector<f32>) -> Vector<f32> {
		self.vectors_have_equal_length(b.clone());
		return Vector::from_vec(self.values.iter().zip(b.values.iter())
		.map(|(u, v)| u.mul_add(a, *v)).collect());
	}

	pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
		Vector::vec_arr_check_length(u);
		let len = u[0].values.len();
		let mut combo: Vector<f32> = Vector::new();
		
		for row in 0..len {
			let mut sum = f32::default();
			for (v, coef) in u.iter().zip(coefs.iter()) {
				sum = (*coef).mul_add(v.values[row], sum);
			}
			combo.values.push(sum);
			combo.rows += 1;
		}
		return combo;
	}

	pub fn dot(&self, v: Vector<f32>) -> f32 {
		self.vectors_have_equal_length(v.clone());
		let mut sum = f32::default();
		for (e1, e2) in self.values.iter().zip(v.values.iter()) {
			sum += *e1 * *e2;
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

// impl Vector<Complex<f32>> {
// 	// pub fn mul_add(&self, a: f32, b: &Vector<f32>) -> Vector<f32> {
// 	// 	self.vectors_have_equal_length(b.clone());
// 	// 	return Vector::from_vec(self.values.iter().zip(b.values.iter())
// 	// 	.map(|(u, v)| u.mul_add(a, *v)).collect());
// 	// }

// 	// pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
// 	// 	Vector::vec_arr_check_length(u);
// 	// 	let len = u[0].values.len();
// 	// 	let mut combo: Vector<f32> = Vector::new();
		
// 	// 	for row in 0..len {
// 	// 		let mut sum = f32::default();
// 	// 		for (v, coef) in u.iter().zip(coefs.iter()) {
// 	// 			sum = (*coef).mul_add(v.values[row], sum);
// 	// 		}
// 	// 		combo.values.push(sum);
// 	// 		combo.rows += 1;
// 	// 	}
// 	// 	return combo;
// 	// }

// 	// pub fn norm_1(&self) -> f32 {
// 	// 	let mut sum: f32 = 0.0;
// 	// 	for el in self.values.iter() {
// 	// 		sum += el.abs();
// 	// 	}
// 	// 	return sum;
// 	// }

// 	pub fn norm(&self) -> f32 {
// 		let mut sum: f32 = 0.0;
// 		for el in self.values.iter() {
// 			sum += el.powf(2.);
// 		}
// 		return sum.powf(0.5);
// 	}

// 	pub fn norm_inf(&self) -> f32 {
// 		let mut max: f32 = 0.0;
// 		for el in self.values.iter() {
// 			if max < el.abs() { max = el.abs(); }
// 		}
// 		return max;
// 	}
// }