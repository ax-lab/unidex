/// Numeric value property for a character. Includes fractions such as the
/// `U+2155 VULGAR FRACTION ONE FIFTH` and numeric values for compatibility
/// characters such as circled numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NumericValue {
	None,
	Integer(i64),
	Rational(i32, i32),
}

impl NumericValue {
	pub fn parse<T: AsRef<str>>(input: T) -> Result<NumericValue, String> {
		let input = input.as_ref();
		if input.len() == 0 {
			return Ok(NumericValue::None);
		}

		let format_error = |err| format!("not a valid numeric value: `{}` -- {}", input, err);

		if let Some(index) = input.find('/') {
			let a = &input[..index];
			let b = &input[index + 1..];
			let a = a.parse::<i32>().map_err(format_error)?;
			let b = b.parse::<i32>().map_err(format_error)?;
			Ok(NumericValue::Rational(a, b))
		} else {
			let value = input.parse::<i64>().map_err(format_error)?;
			Ok(NumericValue::Integer(value))
		}
	}
}

impl std::fmt::Display for NumericValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NumericValue::None => Ok(()),
			NumericValue::Integer(value) => write!(f, "{}", value),
			NumericValue::Rational(a, b) => write!(f, "{}/{}", a, b),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_empty_string_as_none() {
		let input = "";
		assert_eq!(NumericValue::parse(input).unwrap(), NumericValue::None);
	}

	#[test]
	fn parses_integer() {
		let input = "123";
		assert_eq!(
			NumericValue::parse(input).unwrap(),
			NumericValue::Integer(123)
		);

		let input = "-123";
		assert_eq!(
			NumericValue::parse(input).unwrap(),
			NumericValue::Integer(-123)
		);
	}

	#[test]
	fn parses_rational() {
		let input = "2/4";
		assert_eq!(
			NumericValue::parse(input).unwrap(),
			NumericValue::Rational(2, 4)
		);

		let input = "-3/7";
		assert_eq!(
			NumericValue::parse(input).unwrap(),
			NumericValue::Rational(-3, 7)
		);
	}

	#[test]
	fn errors_when_parsing_invalid_input() {
		fn check(input: &'static str, expected_error: &'static str) {
			match NumericValue::parse(input) {
				Err(message) => assert!(
					message.contains(expected_error),
					"expected Err({}) to contain `{}`",
					message,
					expected_error
				),

				other => panic!(
					"parsing {}: expected error with `{}`, was {:?}",
					input, expected_error, other
				),
			}
		}

		check("123x", "not a valid numeric value: `123x`");
		check("1/2x", "not a valid numeric value: `1/2x`");
		check("1x/2", "not a valid numeric value: `1x/2`");
	}

	#[test]
	fn supports_to_string() {
		fn check(input: NumericValue, expected: &'static str) {
			assert_eq!(input.to_string(), expected);
			assert_eq!(NumericValue::parse(input.to_string()).unwrap(), input);
		}
		check(NumericValue::None, "");
		check(NumericValue::Integer(123), "123");
		check(NumericValue::Integer(-123), "-123");
		check(NumericValue::Rational(2, 4), "2/4");
		check(NumericValue::Rational(-3, 6), "-3/6");
	}
}
