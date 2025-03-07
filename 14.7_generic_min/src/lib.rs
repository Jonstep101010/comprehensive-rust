// In this short exercise, you will implement a generic min function that determines the minimum of two values, using the Ord trait.

use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.

#[allow(dead_code)]
fn min<T: Ord>(a: T, b: T) -> T {
	// if a <= b { a } else { b }
	match a.cmp(&b) {
		Ordering::Less | Ordering::Equal => a,
		Ordering::Greater => b,
	}
}

#[test]
fn integers() {
	assert_eq!(min(0, 10), 0);
	assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
	assert_eq!(min('a', 'z'), 'a');
	assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
	assert_eq!(min("hello", "goodbye"), "goodbye");
	assert_eq!(min("bat", "armadillo"), "armadillo");
}
