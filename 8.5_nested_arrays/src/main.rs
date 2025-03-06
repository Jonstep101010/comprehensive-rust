///
/// turn matrix rows into columns:
///
/// matrix first row will be [101, 201, 301]
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
	todo!()
}

fn main() {
	let matrix = [
		[101, 102, 103], // <-- the comment makes rustfmt add a newline
		[201, 202, 203],
		[301, 302, 303],
	];

	dbg!(matrix);
	let transposed = transpose(matrix);
	dbg!(transposed);
}
