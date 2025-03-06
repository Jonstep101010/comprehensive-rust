fn fib(n: u32) -> u32 {
	if n < 2 {
		// The base case.
		n
	} else {
		// The recursive case.
		fib(n - 1) + fib(n - 2)
	}
}

fn main() {
	let n = 20;
	println!("fib({n}) = {}", fib(n));
}
