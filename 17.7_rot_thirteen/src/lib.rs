use std::io::Read;

// only rotate ascii!

#[allow(dead_code)]
struct RotDecoder<R: Read> {
	input: R,
	rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.

impl<R: Read> Read for RotDecoder<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let size = self.input.read(buf)?; // this is the important part ; /
		for b in &mut buf[..size] {
			if b.is_ascii_alphabetic() {
				let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
				// get char offset (index in alphabet), rotate by requirement
				let offset = *b - base + self.rot;
				// reset to new offset at base
				*b = /* rot_b */ offset % 26 + base;
			}
		}
		Ok(size)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn joke() {
		let mut rot = RotDecoder {
			input: "Gb trg gb gur bgure fvqr!".as_bytes(),
			rot: 13,
		};
		let mut result = String::new();
		rot.read_to_string(&mut result).unwrap();
		assert_eq!(&result, "To get to the other side!");
	}

	#[test]
	fn binary() {
		let input: Vec<u8> = (0..=255u8).collect();
		let mut rot = RotDecoder::<&[u8]> {
			input: input.as_ref(),
			rot: 13,
		};
		let mut buf = [0u8; 256];
		assert_eq!(rot.read(&mut buf).unwrap(), 256);
		for i in 0..=255 {
			if input[i] != buf[i] {
				assert!(input[i].is_ascii_alphabetic());
				assert!(buf[i].is_ascii_alphabetic());
			}
		}
	}
}
