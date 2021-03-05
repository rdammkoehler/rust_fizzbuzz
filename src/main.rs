fn fizzbuzz(num:u8) -> String {
	if num % 15 == 0 {
		return String::from("fizzbuzz");
	}
	if num % 3 == 0 {
		return String::from("fizz");
	}
	if num % 5 == 0 {
		return String::from("buzz");
	}
	return num.to_string();
	// how can I concat strings "fizz" + "buzz" so I can break the if-then-else chain?
}

mod tests {

	use crate::fizzbuzz;

	#[test]
	fn test_fizzbuzz_returns_given_number() {
		let result = fizzbuzz(1);

		assert_eq!(result, "1");
	}

	#[test]
	fn test_fizzbuzz_returns_fizz_given_three() {
		let result = fizzbuzz(3);

		assert_eq!(result, "fizz");
	}

	#[test]
	fn test_fizzbuzz_returns_buzz_given_five() {
		let result = fizzbuzz(5);

		assert_eq!(result, "buzz");
	}

	#[test]
	fn test_fizzbuzz_returns_fizzbuzz_given_fifteen() {
		let result = fizzbuzz(15);

		assert_eq!(result, "fizzbuzz");
	}

	#[test]
	fn test_fizzbuzz_returns_two_given_two() {
		let result = fizzbuzz(2);

		assert_eq!(result, "2");
	}

	#[test]
	fn test_fizzbuzz_returns_fizz_given_six() {
		let result = fizzbuzz(6);

		assert_eq!(result, "fizz");
	}

	#[test]
	fn test_fizzbuzz_returns_buzz_given_ten() {
		let result = fizzbuzz(10);

		assert_eq!(result, "buzz");
	}

	#[test]
	fn test_fizzbuzz_returns_fizzbuzz_given_thirty() {
		let result = fizzbuzz(30);

		assert_eq!(result, "fizzbuzz");
	}

}

