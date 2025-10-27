use crate::RandomNumber;
use std::ops::Range;



pub trait Randomizable<T> {

	/// Get a value.
	fn randomizable_value(self) -> T;
}
impl<T:RandomNumber> Randomizable<T> for Range<T> {

	/// Get a value.
	fn randomizable_value(self) -> T {
		T::random_range(self)
	}
}
impl<T:RandomNumber> Randomizable<T> for T {

	/// Get a value.
	fn randomizable_value(self) -> T {
		self
	}
}