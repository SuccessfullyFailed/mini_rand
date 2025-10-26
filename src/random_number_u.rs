#[cfg(test)]
mod test_helpers {
	use std::{ fmt::Debug, ops::Range, thread };
	use crate::RandomNumber;



	pub(super) fn test_not_the_same<T:RandomNumber + PartialEq>() {
		const GENERATED_NUMBER_COUNT:usize = 3200;

		let mut random_numbers:Vec<T> = (0..GENERATED_NUMBER_COUNT).map(|_| T::random()).collect::<Vec<T>>();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), GENERATED_NUMBER_COUNT, "Duplicate numbers were found");
	}

	pub(super) fn test_not_the_same_multi_threaded<T:RandomNumber + PartialEq + Send + Sync + 'static>() {
		const GENERATED_NUMBER_COUNT:usize = 200;
		const THREAD_COUNT:usize = 16;

		let mut random_numbers:Vec<T> =(0..THREAD_COUNT)
			.map(|_| thread::spawn(|| { (0..GENERATED_NUMBER_COUNT).map(|_| T::random()).collect::<Vec<T>>() }) )
			.map(|thread| thread.join())
			.flatten()
			.flatten()
			.collect();
		random_numbers.dedup();
		assert_eq!(random_numbers.len(), GENERATED_NUMBER_COUNT * THREAD_COUNT, "Duplicate numbers were found");
	}

	pub(super) fn test_within_range<T:RandomNumber + PartialOrd + Clone>(range:Range<T>) {
		const GENERATED_NUMBER_COUNT:usize = 3200;

		let random_numbers:Vec<T> = (0..GENERATED_NUMBER_COUNT).map(|_| T::random_range(range.clone())).collect::<Vec<T>>();
		let amount_out_of_range:usize = random_numbers.into_iter().filter(|value:&T| value < &range.start || value > &range.end).count();
		assert_eq!(amount_out_of_range, 0, "Found {amount_out_of_range} values out of range.");
	}

	pub(super) fn test_distribution<T:RandomNumber + Debug + PartialOrd>(half_max:T, factor_range:Range<f32>) {
		test_filtered_factor(|n:&T| n > &half_max, factor_range, |factor| format!("Bad distribution factor: {factor}"));
	}

	pub(super) fn test_filtered_factor<T:RandomNumber + Debug, U:Fn(&T) -> bool, V:Fn(f32) -> String>(filter:U, factor_range:Range<f32>, mismatch_error_handler:V) {
		const GENERATED_NUMBER_COUNT:usize = 3200;

		let random_numbers:Vec<T> = (0..GENERATED_NUMBER_COUNT).map(|_| T::random()).collect::<Vec<T>>();
		let amount:usize = random_numbers.into_iter().filter(filter).count();
		let factor:f32 = amount as f32 / GENERATED_NUMBER_COUNT as f32;
		assert!(factor >= factor_range.start && factor <= factor_range.end, "{}", mismatch_error_handler(factor));
	}
}



// Auto-generate code for unit test on manual activation.
#[cfg(test)]
mod test_generator {
	use std::{ fs, sync::Mutex, thread::sleep, time::Duration};

	static RUNNING_SPECIFICALLY_TEST_GENERATOR:Mutex<bool> = Mutex::new(true);

	#[test]
	fn print_test_code() {

		// Make sure this function was specifically activated.
		sleep(Duration::from_millis(500));
		if !*RUNNING_SPECIFICALLY_TEST_GENERATOR.lock().unwrap() {
			return;
		}

		// Create all required tests.
		let mut tests:Vec<Vec<String>> = Vec::new();
		for type_name in ["u64", "u32", "u16"] {
			tests.push(vec![
				format!("fn test_{type_name}_not_the_same() {{ test_not_the_same::<{type_name}>() }}"),
				format!("fn test_{type_name}_not_the_same_multi_threaded() {{ test_not_the_same_multi_threaded::<{type_name}>() }}"),
				format!("fn test_{type_name}_distribution() {{ test_distribution::<{type_name}>({type_name}::MAX / 2, {}) }}", if type_name == "u8" { "0.2..0.8" } else { "0.35..0.65" }),
				format!("fn test_{type_name}_within_range() {{ test_within_range::<{type_name}>(50..100) }}")
			]);
		}

		// Compile all tests into one string of code.
		let test_code_string:String = format!(
			"#[cfg(test)]\nmod generated_tests {{\n\tuse super::test_helpers::*;\n\n{}\n}}",
			tests.iter().map(|tests_block|
				tests_block.iter().map(|test|
					format!("\t#[test]\n\t{test}")
				).collect::<Vec<String>>().join("\n")
			).collect::<Vec<String>>().join("\n\n\n")
		);

		// Write test code to file.
		let target_file:&'static str = "src/random_number_u.rs";
		let current_content:String = fs::read_to_string(target_file).unwrap();
		let content_parts:Vec<&str> = current_content.split("// AUTO-GENERATED TESTS").collect();
		if content_parts.len() > 1 {
			let trail_size:usize = content_parts[content_parts.len() - 1].len();
			let new_content:String = current_content[..current_content.len() - trail_size].to_string() + "\n" + &test_code_string;
			fs::write(target_file, new_content).unwrap();
		}
	}
	#[test]
	fn cancel_print_test() {
		*RUNNING_SPECIFICALLY_TEST_GENERATOR.lock().unwrap() = false;
	}
}



// AUTO-GENERATED TESTS
#[cfg(test)]
mod generated_tests {
	use super::test_helpers::*;

	#[test]
	fn test_u64_not_the_same() { test_not_the_same::<u64>() }
	#[test]
	fn test_u64_not_the_same_multi_threaded() { test_not_the_same_multi_threaded::<u64>() }
	#[test]
	fn test_u64_distribution() { test_distribution::<u64>(u64::MAX / 2, 0.35..0.65) }
	#[test]
	fn test_u64_within_range() { test_within_range::<u64>(50..100) }


	#[test]
	fn test_u32_not_the_same() { test_not_the_same::<u32>() }
	#[test]
	fn test_u32_not_the_same_multi_threaded() { test_not_the_same_multi_threaded::<u32>() }
	#[test]
	fn test_u32_distribution() { test_distribution::<u32>(u32::MAX / 2, 0.35..0.65) }
	#[test]
	fn test_u32_within_range() { test_within_range::<u32>(50..100) }


	#[test]
	fn test_u16_not_the_same() { test_not_the_same::<u16>() }
	#[test]
	fn test_u16_not_the_same_multi_threaded() { test_not_the_same_multi_threaded::<u16>() }
	#[test]
	fn test_u16_distribution() { test_distribution::<u16>(u16::MAX / 2, 0.35..0.65) }
	#[test]
	fn test_u16_within_range() { test_within_range::<u16>(50..100) }
}