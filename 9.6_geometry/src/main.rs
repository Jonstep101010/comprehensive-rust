// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(vector: &[f64; 3]) -> f64 {
	let mut magnitude: f64 = 0.0;
	for coord in vector {
		magnitude += coord.powf(2.0);
	}
	magnitude.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(vector: &mut [f64; 3]) {
	let mag = magnitude(vector);
	for coord in vector {
		*coord /= mag;
	}
}

// Use the following `main` to test your work.

fn main() {
	println!(
		"Magnitude of a unit vector: {}",
		magnitude(&[0.0, 1.0, 0.0])
	);

	let mut v = [1.0, 2.0, 9.0];
	println!("Magnitude of {v:?}: {}", magnitude(&v));
	normalize(&mut v);
	println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
