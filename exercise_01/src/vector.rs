use core::fmt;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

pub trait ToF64 {
	fn to_f64(&self) -> f64;
}

// Implement the ToF64 trait for f64 itself
impl ToF64 for f64 {
	fn to_f64(&self) -> f64 {
		*self
	}
}

pub trait ToK {
	fn from_f64(value: f64) -> Self;
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> ToK for K
where f64: AddAssign<<K as Mul>::Output>, K: Mul<f64, Output = K> + ToF64 + From<f64> {
	fn from_f64(value: f64) -> K {
		K::from(value)
	}
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> std::ops::Mul<Vector<K>> for f64
where f64: AddAssign<<K as Mul>::Output>, K: Mul<f64, Output = K> + ToF64 + From<f64> {
	type Output = Vector<K>;

	fn mul(self, _rhs: Vector<K>) -> Vector<K> {
		let mut a: Vector<K> = Vector::new();
		for el in _rhs.values.iter() {
			a.values.push(el.clone() * self);
			a.rows += 1;
		}
		a
	}
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> std::ops::Mul<K> for Vector<K>
where f64: AddAssign<<K as Mul>::Output>, K: AddAssign<<K as Mul>::Output> + Mul<K, Output = K>
+ ToF64 + From<f64> + ToK, Vector<K>: Mul<K, Output = Vector<K>> + Add<Vector<K>, Output = Vector<K>> {
	type Output = Vector<K>;

	fn mul(self, _rhs: K) -> Vector<K> {
		let mut a: Vector<K> = Vector::new();
		for el in self.values.iter() {
			a.values.push(el.clone() * _rhs.clone());
			a.rows += 1;
		}
		a
	}
}


impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul + ToK> fmt::Display for Vector<K>
where K: fmt::Display + ToF64 + From<f64> + ToK, f64: AddAssign<<K as Mul>::Output> {
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

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> Vector<K>
where f64: AddAssign<<K as Mul>::Output>, K: ToF64 + From<f64> + ToK {
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

	pub fn print(&self) {
		println!("{}", self);
	}

	pub fn get_rows(&self) -> usize {
		self.rows
	}

	fn vectors_have_equal_length(&self, other: Vector<K>) -> bool {
		if self.rows != other.rows {
			panic!("Vectors must be of the same length.");
		};
		true
	}

	pub fn magnitude(&self) -> f64 {
		let mut sum: f64 = 0.0;
		for el in self.values.iter() {
			sum += el.clone() * el.clone();
		}
		return sum.sqrt();
	}

	pub fn add(&mut self, other: Vector<K>) {
		if self.vectors_have_equal_length(other.clone()) {
			for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
				*a += b.clone();
			}
		}
	}

	pub fn sub(&mut self, other: Vector<K>) {
		if self.vectors_have_equal_length(other.clone()) {
			for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
				*a -= b.clone();
			}
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

	pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
		Vector::vec_arr_check_length(u);
		let len = u[0].values.len();
		let mut combo: Vector<K> = Vector::new();
		let mut sum: f64;

		for row in 0..len {
			sum = 0.0;
			for (v, coef) in u.iter().zip(coefs.iter()) {
				sum = coef.to_f64().mul_add((v.values[row]).to_f64(), sum);
			}
			combo.values.push(K::from_f64(sum));
			combo.rows += 1;
		}
		return combo;
	}
}
