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

impl<K: Clone> Vector<K> {
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
}
