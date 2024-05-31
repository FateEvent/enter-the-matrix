use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<K> {
	values: Vec<Vec<K>>,
	cols: usize,
	rows: usize
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
	pub fn new() -> Self {
		Matrix {
			values: Vec::new(),
			rows: 0,
			cols: 0
		}
	}

	pub fn from(arr: &[&[K]]) -> Self {
		let mut ret = Vec::with_capacity(arr.len());
		for v in arr.iter() {
			ret.push(v.to_vec());
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
