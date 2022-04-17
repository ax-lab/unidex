macro_rules! parse_code {
	($input:expr, $message:expr) => {
		u32::from_str_radix($input, 16).expect(concat!(
			"parsing ",
			$message,
			" is not a valid code"
		))
	};
}

macro_rules! parse_range {
	($input:expr, $message:expr) => {{
		const SEP: &'static str = "..";
		let input = $input;
		let separator =
			input
				.find(SEP)
				.expect(concat!("parsing ", $message, " is not a valid range"));
		let sta = &input[0..separator];
		let end = &input[separator + SEP.len()..];
		let sta = parse_code!(sta, concat!($message, " start"));
		let end = parse_code!(end, concat!($message, " end"));
		(sta, end)
	}};
}

#[cfg(test)]
mod tests {
	#[test]
	fn can_parse_single_code() {
		let value = parse_code!("1234", "entity: field");
		assert_eq!(value, 0x1234);

		let value = parse_code!("ABCD", "entity: field");
		assert_eq!(value, 0xABCD);
	}

	#[test]
	#[should_panic(expected = "parsing entity: field is not a valid code")]
	fn parsing_invalid_code_panics() {
		parse_code!("xx", "entity: field");
	}

	#[test]
	fn can_parse_range() {
		let (a, b) = parse_range!("FF..1234", "entity: field");
		assert_eq!(a, 0xFF);
		assert_eq!(b, 0x1234);

		let (a, b) = parse_range!("1234..ABCD", "entity: field");
		assert_eq!(a, 0x1234);
		assert_eq!(b, 0xABCD);
	}

	#[test]
	#[should_panic(expected = "parsing entity: field is not a valid range")]
	fn parsing_invalid_range_panics() {
		parse_range!("xx", "entity: field");
	}

	#[test]
	#[should_panic(expected = "parsing entity: field start is not a valid code")]
	fn parsing_invalid_range_start_panics() {
		parse_range!("xx..1234", "entity: field");
	}

	#[test]
	#[should_panic(expected = "parsing entity: field end is not a valid code")]
	fn parsing_invalid_range_end_panics() {
		parse_range!("1234..xx", "entity: field");
	}
}
