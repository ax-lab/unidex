pub fn parse_code<S: AsRef<str>>(input: S) -> Result<u32, String> {
	let input = input.as_ref();
	u32::from_str_radix(input.as_ref(), 16)
		.map_err(|err| format!("`{}` is not a valid code ({})", input, err))
}

pub fn parse_range<S: AsRef<str>>(input: S) -> Result<(u32, u32), String> {
	let separator = "..";
	let input = input.as_ref();
	let split_at = input
		.find(separator)
		.ok_or_else(|| format!("`{}` is not a valid range", input))?;
	let sta = &input[0..split_at];
	let end = &input[split_at + separator.len()..];
	let sta = parse_code(sta).map_err(|err| format!("range start {}", err))?;
	let end = parse_code(end).map_err(|err| format!("range end {}", err))?;
	Ok((sta, end))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_parse_single_code() {
		let value = parse_code("1234").unwrap();
		assert_eq!(value, 0x1234);

		let value = parse_code("ABCD").unwrap();
		assert_eq!(value, 0xABCD);
	}

	#[test]
	fn parsing_invalid_code_returns_error() {
		let err = parse_code("xx").unwrap_err();
		assert!(err.contains("`xx` is not a valid code"));
	}

	#[test]
	fn can_parse_range() {
		let (a, b) = parse_range("FF..1234").unwrap();
		assert_eq!(a, 0xFF);
		assert_eq!(b, 0x1234);

		let (a, b) = parse_range("1234..ABCD").unwrap();
		assert_eq!(a, 0x1234);
		assert_eq!(b, 0xABCD);
	}

	#[test]
	fn parsing_invalid_range_returns_error() {
		let err = parse_range("xx").unwrap_err();
		assert!(err.contains("`xx` is not a valid range"))
	}

	#[test]
	fn parsing_invalid_range_start_returns_error() {
		let err = parse_range("xx..1234").unwrap_err();
		assert!(err.contains("range start `xx` is not a valid code"));
	}

	#[test]
	fn parsing_invalid_range_end_returns_error() {
		let err = parse_range("1234..xx").unwrap_err();
		assert!(err.contains("range end `xx` is not a valid code"));
	}
}
