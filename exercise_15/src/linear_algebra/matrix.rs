use core::fmt;

use super::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<K> {
	values: Vec<Vec<K>>,
	cols: usize,
	rows: usize
}

impl std::ops::Index<usize> for Matrix<f32> {
	type Output = Vec<f32>;

	fn index(&self, index: usize) -> &Vec<f32> {
		&self.values[index]
	}
}

impl std::ops::IndexMut<usize> for Matrix<f32> {
	fn index_mut(&mut self, index: usize) -> &mut Vec<f32> {
		&mut self.values[index]
	}
}

impl std::ops::Sub<Matrix<f32>> for Matrix<f32> {
	type Output = Matrix<f32>;

	fn sub(self, _rhs: Matrix<f32>) -> Matrix<f32> {
		self.clone().matrices_are_regular(_rhs.clone());

		let mut m: Matrix<f32> = Matrix::new();
		for (v1, v2) in self.values.iter().zip(_rhs.values.iter()) {
			let mut vec = Vec::new();
			for (a, b) in v1.iter().zip(v2.iter()) {
				vec.push(*a - *b);
			}
			m.values.push(vec);
			m.rows += 1;
		}
		m.cols = m.values.len();
		m

	}
}

impl std::ops::Mul<Matrix<f32>> for f32 {
	type Output = Matrix<f32>;

	fn mul(self, _rhs: Matrix<f32>) -> Matrix<f32> {
		let mut m: Matrix<f32> = Matrix::new();
		for v in _rhs.values.iter() {
			let mut vec = Vec::new();
			for el in v.iter() {
				vec.push(*el * self);
			}
			m.values.push(vec);
			m.rows += 1;
		}
		m.cols = m.values.len();
		m
	}
}

impl fmt::Display for Matrix<f32> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for (i, v) in self.values.iter().enumerate() {
			write!(fmt, "[")?;
			for (j, n) in v.iter().enumerate() {
				write!(fmt, "{}", (n * 100.).round() / 100.)?;
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

impl Matrix<f32> {
	pub fn mul_add(&self, a: f32, b: &Matrix<f32>) -> Matrix<f32> {
		self.clone().matrices_are_regular(b.clone());
		
		let mut m: Matrix<f32> = Matrix::new();
		for (u, v) in self.values.iter().zip(b.values.iter()) {
			let mut vec = Vec::new();
			for (e1, e2) in u.iter().zip(v.iter()) {
				vec.push(e1.mul_add(a, *e2));
			}
			m.values.push(vec);
			m.rows += 1;
		}
		m.cols = m.values.len();
		m
	}

	pub fn mul_vec(&self, vec: Vector<f32>) -> Vector<f32> {
		if self.cols != vec.get_rows() {
			panic!("The number of columns of the matrix must coincide with the number of rows of the vector.")
		}
		
		let mut vector = Vector::new();
		for row in 0..self.rows {
			let capture_row = Vector::capture_row(self.clone(), row);
			vector.push(capture_row.dot(vec.clone()));
		}
		vector
	}

	pub fn mul_mat(&self, mat: Matrix<f32>) -> Matrix<f32> {
		if self.cols != mat.rows {
			panic!("The number of columns of the first matrix must coincide with the number of rows of the second one.")
		}
		
		let mut matrix = Matrix::new();
		for row in 0..self.rows {
			let capture_row = Vector::capture_row(self.clone(), row);
			let mut matrix_row = Vec::new();
			for col in 0..mat.shape().1 {
				let capture_col = Vector::capture_column(mat.clone(), col);
				matrix_row.push(capture_row.dot(capture_col));
			}
			matrix.values.push(matrix_row);
		}
		matrix.set_rows(self.rows);
		matrix.set_cols(mat.cols);
		matrix
	}

	pub fn trace(&self) -> f32 {
		if !self.is_square() {
			panic!("The matrix must be a square matrix.")
		};

		let mut sum: f32 = 0.;
		for entry in 0..self.rows {
			let capture_row = Vector::capture_row(self.clone(), entry);
			sum += capture_row[entry];
		}
		sum
	}

	pub fn transpose(&self) -> Self {

		let mut matrix = Matrix::new();
		for col in 0..self.cols {
			let capture_col = Vector::capture_column(self.clone(), col);
			matrix.push(capture_col);
		}
		matrix.set_rows(self.rows);
		matrix.set_cols(self.cols);
		matrix
	}

	// functions to obtain the row echelon form of a matrix	
	fn row_swap(&mut self, a: usize, b: usize) {
		let tmp = Vector::capture_row(self.clone(), a);

		self[a] = self[b].clone();
		self[b] = tmp.get_values();
	}

	fn row_scl(&mut self, i: usize, scalar: f32) {
		self[i] = (scalar * Vector::from_vec(self[i].clone())).get_values();
	}

	// add to row A a scalar multiple of row B
	fn add_row_multiple(&mut self, a: usize, b: usize, scalar: f32) {
		self[a] = (scalar * Vector::from_vec(self[b].clone()) + Vector::from_vec(self[a].clone())).get_values();
	}

	pub fn row_echelon_form(&self) -> Matrix<f32> {

		let mut matrix = self.clone();
		let mut pivot: usize = 0;
		for col in 0..matrix.cols {
			for row in pivot..matrix.rows {
				if matrix[row][col] != 0.0 {
					if row != pivot {
						matrix.row_swap(pivot, row);
						
					}
					for p_row in pivot + 1..matrix.rows {
						matrix.add_row_multiple(p_row, 0, -1. * matrix[p_row][col] / matrix[pivot][col]);
					}
					pivot += 1;
					break ;
				}
			}
		}
		return matrix;
	}

	pub fn reduced_row_echelon_form(&self) -> Matrix<f32> {

		let mut matrix = self.clone();
		let mut pivot: usize = 0;
		while pivot < matrix.rows {
			let mut d: f32;
			let mut m: f32;

			for row in 0..matrix.rows {
				d = matrix[pivot][pivot];
				m = matrix[row][pivot] / matrix[pivot][pivot];

				if row == pivot {
					matrix.row_scl(row, 1./d);
				} else {
					matrix.add_row_multiple(row, pivot, -1. * m);
				}
				for col in 0..matrix.cols {
					if matrix[row][col] == 0. {
						matrix[row][col] = 0.
					}
				}
			}
			pivot += 1;
		}
		return matrix;
	}

	// functions to compute the determinant of a matrix
	fn determinant_2x2(&self) -> f32 {
		if self.rows != 2 || self.cols != 2 {
			panic!("The matrix must be a 2x2 matrix.")
		};

		return self[0][0] * self[1][1] - self[0][1] * self[1][0]
	}

	fn create_submatrix(&self, index: usize) -> Matrix<f32> {
		let mut matrix = Matrix::new();
		for row in 1..self.rows {
			let mut vec = Vector::new();
			for col in 0..self.cols {
				if index != col {
					vec.push(self[row][col]);
				}
			}
			matrix.push(vec);
			matrix.set_rows(self.rows - 1);
			matrix.set_cols(self.cols - 1);
		}
		return matrix;
	}

	fn determinant_nxn(&self) -> f32 {

		let size = self.rows;
		let mut det: f32 = 0.;
		for i in 0..self.cols {
			let matrix = self.create_submatrix(i);
			if size == 3 {
				if i % 2 == 0 {
					det = self[0][i].mul_add(matrix.determinant_2x2(), det);
				} else {
					det -= self[0][i] * matrix.determinant_2x2();
				}
			} else {
				if i % 2 == 0 {
					det = self[0][i].mul_add(matrix.determinant_nxn(), det);
				} else {
					det -= self[0][i] * matrix.determinant_nxn();
				}
			}
		}
		return det
	}

	pub fn determinant(&self) -> f32 {
		if !self.is_square() {
			panic!("The matrix must be a square matrix.")
		};

		let size = self.rows;

		if size == 1 {
			return self[0][0];
		} else if size == 2 {
			return self.determinant_2x2();
		} else if size >= 3 && size <= 5 {
			return self.determinant_nxn();
		} else {
			panic!("Determinant cannot be calculated for matrices greater than 5x5.")
		};

	}

	// functions to compute the inverse of a matrix
	fn create_adjoint(&self) -> Matrix<f32> {

		let mut min_mat = Matrix::new();
		min_mat.set_rows(self.rows);
		min_mat.set_cols(self.cols);

		let mut neg: f32 = 1.;
		for i in 0..self.rows {
			let mut minor_vec = Vec::new();
			for j in 0..self.cols {
				let mut matrix = Matrix::new();
				matrix.set_rows(self.rows - 1);

				for row in 0..self.rows {
					if i == row {
						continue;
					}
					let mut vec = Vector::new();
					for col in 0..self.cols {
						if j != col {
							vec.push(self[row][col]);
						}
					}
					matrix.push(vec);
				}
				minor_vec.push(matrix.determinant() * neg * if j % 2 == 1 { -1. } else { 1. });
			}
			min_mat.values.push(minor_vec);
			neg *= -1.;
		}
		return min_mat.transpose();
	}

	pub fn inverse(&self) -> Matrix<f32> {
		let det = self.determinant();
		if det == 0. {
			panic!("The inverse of this matrix does not exist.")
		}
		return 1./det * self.create_adjoint();
	}

	pub fn rank(& self) -> usize {
		if self.is_square() {
			let det = self.determinant();
			if det > 0. {
				return self.rows;
			}
		}

		let r_ef = self.row_echelon_form();
		let mut rank: usize = 0;
		let mut col: usize = 0;
		let mut row: usize = 0;
		while row < r_ef.rows && row < r_ef.cols {
			if r_ef[row][col] > 0. {
				rank += 1;
			}
			row += 1;
			col += 1;
		}
		return rank;
	}
}

impl<K: Default + Copy + Clone + std::ops::AddAssign + std::ops::MulAssign
+ std::ops::SubAssign + std::ops::Mul<Output = K>> Matrix<K>
where Vector<K>: std::fmt::Display, Matrix<K>: std::fmt::Display {
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

	pub fn get_values(&self) -> Vec<Vec<K>> {
		self.values.clone()
	}

	pub fn set_cols(&mut self, cols: usize) {
		self.cols = cols;
	}

	pub fn set_rows(&mut self, rows: usize) {
		self.rows = rows;
	}

	pub fn push(&mut self, vec: Vector<K>) {
		self.values.push(vec.get_values());
		self.cols += 1;
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
		self.clone().matrices_are_regular(other.clone());
		for (v1, v2) in self.values.iter_mut().zip(other.values.iter()) {
			for (a, b) in v1.iter_mut().zip(v2.iter()) {
				*a += b.clone();
			}
		}
	}

	pub fn sub(&mut self, other: Matrix<K>) {
		self.clone().matrices_are_regular(other.clone());
		for (v1, v2) in self.values.iter_mut().zip(other.values.iter()) {
			for (a, b) in v1.iter_mut().zip(v2.iter()) {
				*a -= b.clone();
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