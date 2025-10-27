use crate::RandomNumber;



pub trait RandomChoice<T> {

	/// Choose a random option from a list.
	fn choose_random(&self) -> Option<&T>;

	/// Choose a mutable random option from a list.
	fn choose_random_mut(&mut self) -> Option<&mut T>;
}
impl<T> RandomChoice<T> for Vec<T> {

	/// Choose a random option from a list.
	fn choose_random(&self) -> Option<&T> {
		if self.is_empty() {
			None
		} else {
			Some(&self[usize::random_range(0..self.len())])
		}
	}

	/// Choose a mutable random option from a list.
	fn choose_random_mut(&mut self) -> Option<&mut T> {
		if self.is_empty() {
			None
		} else {
			let max:usize = self.len();
			Some(&mut self[usize::random_range(0..max)])
		}
	}
}
impl<T, const U:usize> RandomChoice<T> for [T; U] {

	/// Choose a random option from a list.
	fn choose_random(&self) -> Option<&T> {
		if self.is_empty() {
			None
		} else {
			Some(&self[usize::random_range(0..self.len())])
		}
	}

	/// Choose a mutable random option from a list.
	fn choose_random_mut(&mut self) -> Option<&mut T> {
		if self.is_empty() {
			None
		} else {
			let max:usize = self.len();
			Some(&mut self[usize::random_range(0..max)])
		}
	}
}