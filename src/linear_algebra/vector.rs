use crate::linear_algebra::Matrix;
use crate::linear_algebra::Complex;
use crate::linear_algebra::fmt;
use super::Zero;
use super::MulAdd;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>
}

impl<K> std::ops::Index<usize> for Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display,
K: Copy + Clone + num::Num + std::ops::AddAssign
+ std::ops::SubAssign + std::ops::MulAssign + std::fmt::Display
+ std::ops::Neg<Output = K> + MulAdd<Output = K> {
	type Output = K;

	fn index(&self, index: usize) -> &K {
		&self.values[index]
	}
}

impl<K> std::ops::Sub<Vector<K>> for Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display,
K: Copy + Clone + num::Num + std::ops::AddAssign
+ std::ops::SubAssign + std::ops::MulAssign + std::fmt::Display
+ std::ops::Neg<Output = K> + MulAdd<Output = K> {
	type Output = Vector<K>;

	fn sub(self, _rhs: Vector<K>) -> Vector<K> {
		self.vectors_have_equal_length(&_rhs);
		let mut diff: Vector<K> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			diff.values.push(*a - *b);
		}
		diff
	}
}

impl<K> std::ops::Add<Vector<K>> for Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display,
K: Copy + Clone + num::Num + std::ops::AddAssign
+ std::ops::SubAssign + std::ops::MulAssign + std::fmt::Display
+ std::ops::Neg<Output = K> + MulAdd<Output = K> {
	type Output = Vector<K>;

	fn add(self, _rhs: Vector<K>) -> Vector<K> {
		self.vectors_have_equal_length(&_rhs);
		let mut sum: Vector<K> = Vector::new();
		for (a, b) in self.values.iter().zip(_rhs.values.iter()) {
			sum.values.push(*a + *b);
		}
		sum
	}
}

impl<K> Vector<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display,
K: Copy + Clone + num::Num + std::ops::AddAssign
+ std::ops::SubAssign + std::ops::MulAssign + std::fmt::Display
+ std::ops::Neg<Output = K> + MulAdd<Output = K> {
	pub fn new() -> Self {
		Vector {
			values: Vec::new()
		}
	}
	
	pub fn from(arr: &[K]) -> Self {
		Vector {
			values: arr.to_vec(),
		}
	}

	pub fn from_vec(vec: Vec<K>) -> Self {
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

	pub fn get_values(&self) -> Vec<K> {
		self.values.clone()
	}

	pub fn push(&mut self, val: K) {
		self.values.push(val);
	}

	fn vectors_have_equal_length(&self, other: &Vector<K>) -> bool {
		if self.rows() != other.rows() {
			panic!("Vectors must be of the same length.");
		};
		true
	}

	pub fn add(&mut self, other: Vector<K>) {
		self.vectors_have_equal_length(&other);
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a += b.clone();
		}
	}

	pub fn sub(&mut self, other: Vector<K>) {
		self.vectors_have_equal_length(&other);
		for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
			*a -= b.clone();
		}
	}

	pub fn scl(&mut self, scalar: K) {
		for el in self.values.iter_mut() {
			*el *= scalar.clone();
		}
	}

	pub fn mul_add(&self, a: K, b: &Vector<K>) -> Vector<K> {
		self.vectors_have_equal_length(&b);
		return Vector::from_vec(self.values.iter().zip(b.values.iter())
		.map(|(u, v)| u.mul_add(a, *v)).collect());
	}

	fn vec_arr_check_length(u: &[Vector<K>]) -> bool {
		for v in u {
			if !Vector::vectors_have_equal_length(&u[0], &v) {
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
			write!(fmt, "{}", (n * 1000.).round() / 1000.)?;
			write!(fmt, "]")?;
			write!(fmt, "\n")?;
		}
		println!("The vector has {} rows.", self.rows());
		Ok(())
	}
}

impl Vector<f32> {
	pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
		Vector::vec_arr_check_length(u);
		let len = u[0].values.len();
		let mut combo: Vector<f32> = Vector::new();
		
		for row in 0..len {
			let mut sum = f32::zero();
			for (v, coef) in u.iter().zip(coefs.iter()) {
				sum = (*coef).mul_add(v.values[row], sum);
			}
			combo.values.push(sum);
		}
		return combo;
	}

	pub fn dot(&self, v: &Vector<f32>) -> f32 {
		self.vectors_have_equal_length(&v);
		let mut sum = f32::zero();
		for (e1, e2) in self.values.iter().zip(v.values.iter()) {
			sum += *e1 * *e2;
		}
		sum
	}

	pub fn norm_1(&self) -> f32 {
		let mut sum = f32::zero();
		for el in self.values.iter() {
			sum += el.abs();
		}
		return sum;
	}

	pub fn norm(&self) -> f32 {
		let mut sum = f32::zero();
		for el in self.values.iter() {
			sum += el.powf(2.);
		}
		return sum.powf(0.5);
	}

	pub fn norm_inf(&self) -> f32 {
		let mut max = f32::zero();
		for el in self.values.iter() {
			if max < el.abs() { max = el.abs(); }
		}
		return max;
	}

	pub fn angle_cos(&self, u: &Vector<f32>) -> f32 {
		self.dot(&u) / (self.norm() * u.norm())
	}
}

impl std::ops::Mul<Vector<Complex<f32>>> for f32 {
	type Output = Vector<Complex<f32>>;

	fn mul(self, _rhs: Vector<Complex<f32>>) -> Vector<Complex<f32>> {
		let mut a: Vector<Complex<f32>> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(*el * self);
		}
		a
	}
}

impl std::ops::Mul<Vector<Complex<f32>>> for Complex<f32> {
	type Output = Vector<Complex<f32>>;

	fn mul(self, _rhs: Vector<Complex<f32>>) -> Vector<Complex<f32>> {
		let mut a: Vector<Complex<f32>> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(*el * self);
		}
		a
	}
}

impl fmt::Display for Vector<Complex<f32>> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for n in self.values.iter() {
			let mut c: Complex<f32> = *n;
			c.re = (c.re * 1000.).round() / 1000.;
			c.im = (c.im * 1000.).round() / 1000.;
			write!(fmt, "[")?;
			write!(fmt, "{}", c)?;
			write!(fmt, "]")?;
			write!(fmt, "\n")?;
		}
		println!("The vector has {} rows.", self.rows());
		Ok(())
	}
}

impl Vector<Complex<f32>> {
	pub fn dot(&self, v: &Vector<Complex<f32>>) -> f32 {
		self.vectors_have_equal_length(&v);
		let mut sum = f32::zero();
		for (e1, e2) in self.values.iter().zip(v.values.iter()) {
			sum += (*e1 * e2.conj()).re;
		}
		sum
	}

	pub fn norm_1(&self) -> f32 {
		let mut sum = f32::zero();
		for el in self.values.iter() {
			sum += el.norm();
		}
		return (sum * 1000.).round() / 1000.;
	}

	pub fn norm(&self) -> f32 {
		let mut sum = f32::zero();
		for el in self.values.iter() {
			sum += el.norm().powf(2.);
		}
		return (sum.powf(0.5) * 1000.).round() / 1000.;
	}

	pub fn norm_inf(&self) -> f32 {
		let mut max = f32::zero();
		for el in self.values.iter() {
			if max < el.norm() { max = el.norm(); }
		}
		return (max * 1000.).round() / 1000.;
	}

	pub fn angle_cos(&self, u: &Vector<Complex<f32>>) -> f32 {
		self.dot(&u) / (self.norm() * u.norm())
	}
}