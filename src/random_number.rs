use std::{ ops::Range, time::Duration };
use crate::generate_random_u64;



pub trait RandomNumber:Sized {
	
	/// Get a random value.
	fn random() -> Self;
	
	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self;
}



/* INTEGER IMPLEMENTATIONS */

impl RandomNumber for u128 {

	/// Get a random value.
	fn random() -> Self {
		u128::random_range(u128::MIN..u128::MAX)
	}

	/// Get a random value between the given values.
	fn random_range(range:Range<Self>) -> Self {
		let available_range_size:u128 = range.end - range.start;
		let seed:u128 = generate_random_u64() as u128 + ((generate_random_u64() as u128) << 64);
		range.start + (seed % available_range_size)
	}

}

macro_rules! impl_randomizable_uint {
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
impl_randomizable_uint!(usize);
impl_randomizable_uint!(u64);
impl_randomizable_uint!(u32);
impl_randomizable_uint!(u16);
impl_randomizable_uint!(u8);

macro_rules! impl_randomizable_iint {
	($type:ty, $type_unsigned:ty) => {
		impl RandomNumber for $type {
	
			/// Get a random value.
			fn random() -> Self {
				<$type>::random_range(<$type>::MIN..<$type>::MAX)
			}

			/// Get a random value between the given values.
			fn random_range(range:Range<Self>) -> Self {
				let range_unsigned:[$type_unsigned; 2] = {
					[range.start, range.end].map(|value|
						if value < 0 {
							(value + <$type>::MAX + 1) as $type_unsigned
						} else {
							(value as $type_unsigned) + (<$type>::MAX as $type_unsigned)
						}
					)
				};
				let random_unsigned:$type_unsigned = <$type_unsigned>::random_range(range_unsigned[0]..range_unsigned[1]);
				if random_unsigned < <$type>::MAX as $type_unsigned {
					random_unsigned as $type
				} else {
					(random_unsigned - <$type>::MAX as $type_unsigned) as $type
				}
			}
		}
	};
}
impl_randomizable_iint!(i128, u128);
impl_randomizable_iint!(isize, usize);
impl_randomizable_iint!(i64, u64);
impl_randomizable_iint!(i32, u32);
impl_randomizable_iint!(i16, u16);
impl_randomizable_iint!(i8, u8);



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