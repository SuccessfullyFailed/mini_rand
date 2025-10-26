use std::{ ops::Range, time::Duration };
use crate::generate_random_u64;



pub trait RandomNumber:Sized {
	
	/// Get a random value.
	fn random() -> Self;
	
	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self;
}



/* INTEGER IMPLEMENTATIONS */

macro_rules! impl_randomizable_int {
	($type:ty) => {
		impl RandomNumber for $type {
	
			/// Get a random value.
			fn random() -> Self {
				<$type>::random_range(<$type>::MIN..<$type>::MAX)
			}

			/// Get a random value between the given values.
			fn random_range(range:Range<Self>) -> Self {
				let available_range_size:$type = range.end - range.start;
				let seed:$type = generate_random_u64() as $type;
				range.start + (seed % available_range_size)
			}
		}
	};
}
impl_randomizable_int!(u64);
impl_randomizable_int!(u32);
impl_randomizable_int!(u16);
impl_randomizable_int!(u8);



/* FLOAT IMPLEMENTATIONS */

macro_rules! impl_randomizable_float {
	($type:ty) => {
		impl RandomNumber for $type {

			/// Get a random value.
			fn random() -> Self {
				(u32::random() as $type) / (u32::MAX as $type) * <$type>::MAX
			}

			/// Get a random value between the given values.
			fn random_range(range:Range<Self>) -> Self {
				range.start + ((u32::random() as $type) / (u32::MAX as $type) * (range.end - range.start))
			}
		}
	};
}
impl_randomizable_float!(f64);
impl_randomizable_float!(f32);



/* OTHER TYPE IMPLEMENTATIONS */

impl RandomNumber for bool {

	/// Get a random value.
	fn random() -> Self {
		u8::random_range(0..1) == 0
	}

	/// Get a random value between the given values.
	fn random_range(_range:Range<Self>) -> Self {
		Self::random()
	}
}

impl RandomNumber for Duration {

	/// Get a random value.
	fn random() -> Self {
		Duration::from_millis(u64::random())
	}

	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self {
		Duration::from_millis(u64::random_range(range.start.as_millis() as u64..range.end.as_millis() as u64))
	}
}

impl RandomNumber for char {

	/// Get a random value.
	fn random() -> Self {
		u8::random() as char
	}

	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self {
		u8::random_range(range.start as u8..range.end as u8) as char
	}
}