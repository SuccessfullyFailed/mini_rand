#[cfg(test)]
mod tests {
	use std::{ ops::Range, thread };
	use crate::generate_random_u64;



	#[test]
	fn test_numbers_not_the_same() {
		const GENERATED_NUMBER_COUNT:usize = 3200;

		let mut random_numbers:Vec<u64> = (0..GENERATED_NUMBER_COUNT).map(|_| generate_random_u64()).collect::<Vec<u64>>();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), GENERATED_NUMBER_COUNT, "Duplicate numbers were found");
	}

	#[test]
	fn test_numbers_not_the_same_multi_threaded() {
		const GENERATED_NUMBER_COUNT:usize = 2000;
		const THREAD_COUNT:usize = 16;

		let mut random_numbers:Vec<u64> =(0..THREAD_COUNT)
			.map(|_| thread::spawn(|| { (0..GENERATED_NUMBER_COUNT).map(|_| generate_random_u64()).collect::<Vec<u64>>() }) )
			.map(|thread| thread.join())
			.flatten()
			.flatten()
			.collect();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), GENERATED_NUMBER_COUNT * THREAD_COUNT, "Duplicate numbers were found");
	}

	#[test]
	fn test_numbers_distribution() {
		const GENERATED_NUMBER_COUNT:usize = 3200;
		const HALF:u64 = u64::MAX / 2;
		const ACCEPTABLE_RANGE:Range<f32> = 0.35..0.65;

		let random_numbers:Vec<u64> = (0..GENERATED_NUMBER_COUNT).map(|_| generate_random_u64()).collect::<Vec<u64>>();
		let numbers_above_half:usize = random_numbers.iter().filter(|number| *number > &HALF).count();
		let numbers_above_half_factor:f32 = numbers_above_half as f32 / GENERATED_NUMBER_COUNT as f32;
		assert!(ACCEPTABLE_RANGE.contains(&numbers_above_half_factor), "Bad distribution factor: {numbers_above_half_factor}");
	}
}