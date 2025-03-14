/*
The Luhn algorithm is used to validate credit card numbers.
The algorithm takes a string as input and does the following to validate the credit card number:

	Ignore all spaces. Reject numbers with fewer than two digits.

	Moving from right to left, double every second digit: for the number 1234, we double 3 and 1. For the number 98765, we double 6 and 8.

	After doubling a digit, sum the digits if the result is greater than 9. So doubling 7 becomes 14 which becomes 1 + 4 = 5.

	Sum all the undoubled and doubled digits.

	The credit card number is valid if the sum ends with 0.

The provided code provides a buggy implementation of the luhn algorithm, along with two basic unit tests that confirm that most of the algorithm is implemented correctly.
*/

pub fn luhn(cc_number: &str) -> bool {
	let mut sum = 0;
	let mut double = false;
	let mut digits = 0;

	for c in cc_number.chars().rev() {
		if let Some(digit) = c.to_digit(10) {
			digits += 1;
			if double {
				let double_digit = digit * 2;
				sum += if double_digit > 9 {
					double_digit - 9
				} else {
					double_digit
				};
			} else {
				sum += digit;
			}
			double = !double;
		} else if c.is_whitespace() && digits >= 3 {
			// ignore whitespace, but only if it is not trailing
			continue;
		} else {
			return false;
		}
	}

	digits >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_valid_cc_number() {
		assert!(luhn("4263 9826 4026 9299"));
		assert!(luhn("4539 3195 0343 6467"));
		assert!(luhn("7992 7398 713"));
	}

	#[test]
	fn test_invalid_cc_number() {
		assert!(!luhn("4223 9826 4026 9299"));
		assert!(!luhn("4539 3195 0343 6476"));
		assert!(!luhn("8273 1232 7352 0569"));
		assert!(!luhn("8273 0569"));
	}
	#[test]
	fn test_invalid_length_cc_number() {
		assert!(!luhn("1234 5678 9012 3456 7890"));
		assert!(!luhn("0"));
		assert!(!luhn(" 0"));
		assert!(!luhn("   0    "));
		assert!(!luhn("0 0 "));
		assert!(!luhn("   0   0    "));
		assert!(!luhn("   0   0"));
	}
	#[test]
	fn test_nondigit_cc_number() {
		assert!(!luhn("some text"));
		assert!(!luhn("4223 9826 a026 9299"));
	}
	#[test]
	fn test_empty_cc_number() {
		assert!(!luhn(""));
		assert!(!luhn("  "));
		assert!(!luhn("     "));
		assert!(!luhn("     	"));
	}
}
