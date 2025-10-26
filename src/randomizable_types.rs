use std::{ ops::Range, time::Duration };
use crate::generate_random_u64;



pub trait Randomizable:Sized {
	
	/// Get a random value.
	fn random() -> Self;
	
	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self;
}



/* INTEGER IMPLEMENTATIONS */

macro_rules! impl_randomizable_int {
	($($type:ty),*) => {
		$(
			impl Randomizable for $type {
		
				/// Get a random value.
				fn random() -> Self {
					Self::random_range(<$type>::MIN..<$type>::MAX)
				}

				/// Get a random value between the given values.
				fn random_range(range:Range<Self>) -> Self {
					range.start + ((generate_random_u64() as $type) % (range.end - range.start))
				}
			}
		)*
	};
}
impl_randomizable_int!(u64, i64, u32, i32, u16, i16, u8, i8);



/* FLOAT IMPLEMENTATIONS */

macro_rules! impl_randomizable_float {
	($type:ty, $seed_type:ty) => {
		impl Randomizable for $type {

			/// Get a random value.
			fn random() -> Self {
				Self::random_range(<$type>::MIN..<$type>::MAX)
			}

			/// Get a random value between the given values.
			fn random_range(range:Range<Self>) -> Self {
				let random_fact:$type = (<$seed_type>::random() as $type) / ((<$seed_type>::MAX as $type) + 1.0);
				range.start + (random_fact * (range.end - range.start))
			}
		}
	};
}
impl_randomizable_float!(f64, u64);
impl_randomizable_float!(f32, u32);



/* OTHER TYPE IMPLEMENTATIONS */

impl Randomizable for bool {

	/// Get a random value.
	fn random() -> Self {
		u8::random_range(0..1) == 0
	}

	/// Get a random value between the given values.
	fn random_range(_range:Range<Self>) -> Self {
		Self::random()
	}
}

impl Randomizable for Duration {

	/// Get a random value.
	fn random() -> Self {
		Duration::from_millis(u64::random())
	}

	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self {
		Duration::from_millis(u64::random_range(range.start.as_millis() as u64..range.end.as_millis() as u64))
	}
}

impl Randomizable for char {

	/// Get a random value.
	fn random() -> Self {
		u8::random() as char
	}

	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self {
		u8::random_range(range.start as u8..range.end as u8) as char
	}
}