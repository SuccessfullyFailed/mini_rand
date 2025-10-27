#[cfg(test)]
mod test_helpers {
	use crate::Randomizable;



	#[test]
	fn test_range_numbers_not_the_same() {
		const GENERATED_NUMBER_COUNT:usize = 3200;

		let mut random_numbers:Vec<u64> = (0..GENERATED_NUMBER_COUNT).map(|_| (0..u64::MAX).randomizable_value()).collect::<Vec<u64>>();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), GENERATED_NUMBER_COUNT, "Duplicate numbers were found");
	}

	#[test]
	fn test_literal_numbers_are_the_same() {
		const GENERATED_NUMBER_COUNT:usize = 3200;
		const SAMPLE_NUMBER:u64 = 120;

		let mut random_numbers:Vec<u64> = (0..GENERATED_NUMBER_COUNT).map(|_| SAMPLE_NUMBER.randomizable_value()).collect::<Vec<u64>>();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), 1, "Numbers weren't all the same");
		assert_eq!(random_numbers[0], SAMPLE_NUMBER, "Found number is somehow wrong");
	}

	#[test]
	fn test_unimplemented_type() {
		#[derive(PartialEq, Debug)]
		struct CustomStruct(i8);

		assert_eq!(CustomStruct(10).randomizable_value(), CustomStruct(10));
	}
}