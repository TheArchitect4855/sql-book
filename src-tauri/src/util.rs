pub fn to_hex(bytes: &[u8]) -> String {
	let mut buffer = String::with_capacity(bytes.len() * 3);
	for b in bytes {
		buffer.push_str(
			&format!("{:x} ", b)
		);
	}

	buffer
}
