/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
#[allow(unused)]
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
	// basic loop based implementation
	let mut loop_result = Vec::with_capacity(values.len());
	for n in 0..values.len() {
		loop_result.push(
			values[
			// circular index: modulo to wrap around when needed
			(n + offset) % values.len()]
			// current index
				- values[n],
		);
	}

	// iterator based implementation: adapted from solution
	// all zeros are overwritten without wrapping by offset using skip after cycling
	let iter_result = values
		.iter()
		.zip(values.iter().cycle().skip(offset))
		.map(|(&zero, &cycled)| cycled - zero)
		.collect();

	assert_eq!(loop_result, iter_result);
	iter_result
}

#[test]
fn test_offset_one() {
	assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
	assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
	assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
	assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
	assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
	assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
	assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
	assert_eq!(offset_differences(1, vec![0]), vec![0]);
	assert_eq!(offset_differences(1, vec![1]), vec![0]);
	let empty: Vec<i32> = vec![];
	assert_eq!(offset_differences(1, empty), vec![]);
}
