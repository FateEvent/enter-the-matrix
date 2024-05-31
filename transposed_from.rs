// the Matrix::from is TRANSPOSED

use core::fmt;
use core::ops::AddAssign;
use core::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<K> {
	values: Vec<Vec<K>>,
	cols: usize,
	rows: usize
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> fmt::Display for Vector<K>
where K: fmt::Display, f64: AddAssign<<K as Mul>::Output> {
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
+ std::ops::SubAssign + std::ops::MulAssign + std::ops::Mul> Vector<K> where f64: AddAssign<<K as Mul>::Output> {
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
}

impl<K> fmt::Display for Matrix<K>
where K: fmt::Display {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		println!("The shape of the matrix is {} x {}", self.rows, self.cols);
		for (i, v) in self.values.iter().enumerate() {
			write!(fmt, "[")?;
			for (j, n) in v.iter().enumerate() {
				write!(fmt, "{}", n)?;
				if j < v.len() - 1 {
					write!(fmt, " ")?;
				}
			}
			write!(fmt, "]")?;
			if i < self.values.len() - 1 {
				write!(fmt, "\n")?;
			}
		}
		Ok(())
	}
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K>
+ std::ops::SubAssign + std::ops::MulAssign> Matrix<K> {
	pub fn from(arr: &[&[K]]) -> Self {
		let mut ret = Vec::with_capacity(arr.len());
		for i in 0..arr.len() {
			let mut new = Vec::new();
			for v in arr.iter() {
				new.push(v[i].clone());
			}
			ret.push(new);
		}
		let matrix = Matrix {
			values: ret,
			rows: arr.len(),
			cols: arr[0].len()
		};
		matrix.is_regular();
		matrix
	}

	pub fn print(&self) {
		println!("{}", self);
	}

	pub fn shape(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}

	pub fn is_square(&self) -> bool {
		self.cols == self.rows
	}

	fn is_regular(&self) -> bool {
		let mut i = 1;

		while i < self.values.len() {
			if self.values[i].len() != self.values[0].len() {
				panic!("The matrix is not regular.");
			};
			i += 1;
		}
		true
	}

	fn matrices_are_equally_sized(&self, other: Matrix<K>) -> bool {
		if self.cols != other.cols {
			panic!("The matrices must have the same number of columns.");
		};
		if self.rows != other.rows {
			panic!("The matrices must have the same number of rows.");
		};
		true
	}

	fn matrices_are_regular(self, other: Matrix<K>) -> bool {
		return self.is_regular() && other.is_regular()
			&& self.matrices_are_equally_sized(other);
	}

	pub fn add(&mut self, other: Matrix<K>) {
		if self.clone().matrices_are_regular(other.clone()) {
			for (v1, v2) in self.values.iter_mut().zip(other.values.iter()) {
				for (a, b) in v1.iter_mut().zip(v2.iter()) {
					*a += b.clone();
				}
			}
		}
	}

	pub fn sub(&mut self, other: Matrix<K>) {
		if self.clone().matrices_are_regular(other.clone()) {
			for (v1, v2) in self.values.iter_mut().zip(other.values.iter()) {
				for (a, b) in v1.iter_mut().zip(v2.iter()) {
					*a -= b.clone();
				}
			}
		}
	}

	pub fn scl(&mut self, scalar: K) {
		for v in self.values.iter_mut() {
			for el in v.iter_mut() {
				*el *= scalar.clone();
			}
		}
	}
}
