use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Vector<K> {
	values: Vec<K>,
	rows: usize
}

#[derive(Debug, PartialEq)]
pub struct Matrix<K> {
	values: Vec<Vec<K>>,
	cols: usize,
	rows: usize
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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

impl<K: Clone + std::fmt::Display + std::ops::AddAssign + std::ops::Add<Output = K> > Vector<K> {
	pub fn new(arr: &[K]) -> Self {
		Vector {
			values: arr.to_vec(),
			rows: arr.len()
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

	pub fn add(self, other: Vector<K>) -> Vector<K> {
		let mut sum: Vector<K> = Vector {
			values: Vec::new(),
			rows: self.rows
		};

		// Ensure that both vectors have the same length
		assert_eq!(self.values.len(), other.values.len(), "Vectors must be of the same length");

		for (a, b) in self.values.iter().zip(other.values.iter()) {
			sum.values.push(a.clone() + b.clone());
		}
		return sum
	}
}

impl<K> fmt::Display for Matrix<K>
where K: fmt::Display {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(fmt, "[")?;
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
		write!(fmt, "]")?;
		Ok(())
	}
}

impl<K: Clone + std::fmt::Display + std::ops::AddAssign> Matrix<K> {
	pub fn new(arr: &[&[K]]) -> Self {
		let mut ret = Vec::with_capacity(arr.len());
		for v in arr.iter() {
			ret.push(v.to_vec());
		}
		Matrix {
			values: ret,
			rows: arr.len(),
			cols: arr[0].len()
		}
	}

	pub fn from(arr: &[&[K]]) -> Self {
		let mut ret = Vec::with_capacity(arr.len());
		for v in arr.iter() {
			ret.push(v.to_vec());
		}
		Matrix {
			values: ret,
			rows: arr.len(),
			cols: arr[0].len()
		}
	}

	pub fn print(&self) {
		println!("{}", self);
	}
}
