use super::data::*;
use super::parse::parse_code;

#[derive(Debug, Eq, PartialEq)]
pub struct UnicodeData<'a> {
	/// Codepoint value. Note that for codepoint ranges this can represent the
	/// start or end of a range of codepoints.
	pub code: u32,

	/// Character name.
	pub name: &'a str,

	/// General category for the character.
	pub category: Category,

	/// The classes used for the Canonical Ordering Algorithm in the Unicode
	/// standard.
	pub combining_class: u32,

	/// Bidirectional category for this character.
	pub bidi: Bidi,

	/// Decomposition mapping for the character.
	pub decomposition: Option<Decomposition>,

	/// Decimal digit value, if the character has the decimal digit property.
	pub decimal_value: DecimalValue,

	/// Digit value, if the character represents a digit, not necessarily a
	/// decimal digit.
	pub digit_value: DigitValue,

	/// Numeric value, if the character has the numeric property.
	pub numeric_value: NumericValue,

	/// If the character has been identified as a "mirrored" character in
	/// bidirectional text this is [`Mirrored::Yes`].
	pub mirrored: Mirrored,

	/// Old name as published in Unicode 1.0. This name is only provided
	/// when it is significantly different from the Unicode 3.0 name for
	/// the character.
	pub unicode_old_name: &'a str,

	/// This is the ISO 10646 comment field. It is in parantheses in the 10646
	/// names list.
	pub iso_10646_comment: &'a str,

	/// Uppercase mapping for this character.
	pub uppercase_mapping: CaseMapping,

	/// Lowercase mapping for this character.
	pub lowercase_mapping: CaseMapping,

	/// Titlecase mapping for this character.
	pub titlecase_mapping: CaseMapping,
}

impl<'a> UnicodeData<'a> {
	pub fn parse(input: &'a str) -> Result<Self, String> {
		//----[ parsing helpers ]---------------------------------------------//

		let error_message =
			|msg: &str| format!("parsing unicode data: {} -- row: `{}`", msg, input);

		let field_error = |field: &str, value: &str| {
			let message = format!("invalid {} `{}`", field, value);
			error_message(&message)
		};

		let parse_u32 = |field_name: &str, value: &str| {
			value
				.parse::<u32>()
				.map_err(|_| field_error(field_name, value))
		};

		//----[ row parsing ]-------------------------------------------------//

		if input.len() == 0 {
			return Err(error_message("empty input"));
		}

		let mut fields = input.split(';');
		let mut next_field = || {
			fields
				.next()
				.ok_or_else(|| error_message("invalid row format"))
		};

		let code = next_field()?;
		let name = next_field()?;
		let category = next_field()?;
		let combining_class = next_field()?;
		let bidi = next_field()?;
		let decomposition = next_field()?;
		let decimal_value = next_field()?;
		let digit_value = next_field()?;
		let numeric_value = next_field()?;
		let mirrored = next_field()?;
		let unicode_old_name = next_field()?;
		let iso_10646_comment = next_field()?;
		let uppercase_mapping = next_field()?;
		let lowercase_mapping = next_field()?;
		let titlecase_mapping = next_field()?;

		if fields.count() != 0 {
			return Err(error_message("invalid row format"));
		}

		//----[ field parsing ]-----------------------------------------------//

		let code = parse_code(code).map_err(|err| error_message(&err))?;

		if name.trim().len() == 0 {
			return Err(error_message("empty name"));
		}

		let category =
			Category::parse(category).ok_or_else(|| field_error("category", category))?;

		let combining_class = parse_u32("combining class", combining_class)?;

		let bidi = Bidi::parse(bidi).ok_or_else(|| field_error("bidirectional category", bidi))?;

		let decomposition = if decomposition.len() > 0 {
			Some(Decomposition::parse(decomposition).map_err(|err| {
				format!("{} ({})", field_error("decomposition", decomposition), err)
			})?)
		} else {
			None
		};

		let decimal_value = if decimal_value.len() > 0 {
			DecimalValue::Some(parse_u32("decimal value", decimal_value)?)
		} else {
			DecimalValue::None
		};

		let digit_value = if digit_value.len() > 0 {
			DigitValue::Some(parse_u32("digit value", digit_value)?)
		} else {
			DigitValue::None
		};

		let numeric_value = NumericValue::parse(numeric_value)
			.map_err(|err| format!("{} ({})", field_error("numeric value", numeric_value), err))?;

		let mirrored = match mirrored {
			"Y" => Mirrored::Yes,
			"N" => Mirrored::No,
			_ => return Err(field_error("mirrored value", mirrored)),
		};

		let parse_case = |name: &str, input: &str| -> Result<_, String> {
			if input.len() > 0 {
				Ok(CaseMapping::Some(
					u32::from_str_radix(input, 16).map_err(|_| field_error(name, input))?,
				))
			} else {
				Ok(CaseMapping::None)
			}
		};

		let uppercase_mapping = parse_case("uppercase mapping", uppercase_mapping)?;
		let lowercase_mapping = parse_case("lowercase mapping", lowercase_mapping)?;
		let titlecase_mapping = parse_case("titlecase mapping", titlecase_mapping)?;

		//----[ result ]------------------------------------------------------//

		let output = UnicodeData {
			code,
			name,
			category,
			combining_class,
			bidi,
			decomposition,
			decimal_value,
			digit_value,
			numeric_value,
			mirrored,
			unicode_old_name,
			iso_10646_comment,
			uppercase_mapping,
			lowercase_mapping,
			titlecase_mapping,
		};
		Ok(output)
	}
}

impl<'a> std::fmt::Display for UnicodeData<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:04X}", self.code)?;
		write!(f, ";{}", self.name)?;
		write!(f, ";{}", self.category)?;
		write!(f, ";{}", self.combining_class)?;
		write!(f, ";{}", self.bidi)?;
		if let Some(decomposition) = &self.decomposition {
			write!(f, ";{}", decomposition)?;
		} else {
			write!(f, ";")?;
		}
		if let DecimalValue::Some(decimal_value) = self.decimal_value {
			write!(f, ";{}", decimal_value)?;
		} else {
			write!(f, ";")?;
		}
		if let DigitValue::Some(digit_value) = self.digit_value {
			write!(f, ";{}", digit_value)?;
		} else {
			write!(f, ";")?;
		}
		write!(f, ";{}", self.numeric_value)?;
		write!(f, ";{}", self.mirrored)?;
		write!(f, ";{}", self.unicode_old_name)?;
		write!(f, ";{}", self.iso_10646_comment)?;
		write!(f, ";{}", self.uppercase_mapping)?;
		write!(f, ";{}", self.lowercase_mapping)?;
		write!(f, ";{}", self.titlecase_mapping)?;
		Ok(())
	}
}

/// Values for the decimal digit value property for a character.
///
/// See also [`DigitValue`], [`NumericValue`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecimalValue {
	None,
	Some(u32),
}

/// Value for the digit value property for a character that represents a digit,
/// not necessarily a decimal digit.
///
/// See also [`DecimalValue`], [`NumericValue`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DigitValue {
	None,
	Some(u32),
}

/// Mirrored property for characters in bidirectional text. The list of
/// mirrored characters is printed in Chapter 4 of the Unicode Standard.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mirrored {
	No,
	Yes,
}

impl std::fmt::Display for Mirrored {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Mirrored::No => "N",
				Mirrored::Yes => "Y",
			}
		)
	}
}

/// If a character is part of an alphabet with case distinctions, and has a
/// case equivalent, this will be the value.
///
/// These mappings are always one-to-one, not one-to-many or many-to-one. It
/// also doesn't contain information about context-sensitive case mappings
/// (i.e. `SpecialCasing.txt`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CaseMapping {
	None,
	Some(u32),
}

impl std::fmt::Display for CaseMapping {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CaseMapping::Some(value) => write!(f, "{:04X}", value),
			CaseMapping::None => write!(f, ""),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::DecompositionTag;

	#[test]
	fn can_create_new() {
		let decomposition = Decomposition {
			tag: Some(DecompositionTag::Font),
			codes: vec![10, 20, 30],
		};
		let entry = UnicodeData {
			code: 0x12AB,
			name: "char name",
			category: Category::LetterLowercase,
			combining_class: 230,
			bidi: Bidi::L,
			decomposition: Some(decomposition.clone()),
			decimal_value: DecimalValue::Some(1),
			digit_value: DigitValue::Some(1),
			numeric_value: NumericValue::Rational(1, 5),
			mirrored: Mirrored::Yes,
			unicode_old_name: "unicode 1.0 name",
			iso_10646_comment: "ISO 10646 comment",
			uppercase_mapping: CaseMapping::Some(1),
			lowercase_mapping: CaseMapping::Some(2),
			titlecase_mapping: CaseMapping::None,
		};
		assert_eq!(entry.code, 0x12AB);
		assert_eq!(entry.name, "char name");
		assert_eq!(entry.category, Category::LetterLowercase);
		assert_eq!(entry.combining_class, 230);
		assert_eq!(entry.bidi, Bidi::L);
		assert_eq!(entry.decomposition, Some(decomposition));
		assert_eq!(entry.decimal_value, DecimalValue::Some(1));
		assert_eq!(entry.digit_value, DigitValue::Some(1));
		assert_eq!(entry.numeric_value, NumericValue::Rational(1, 5));
		assert_eq!(entry.mirrored, Mirrored::Yes);
		assert_eq!(entry.unicode_old_name, "unicode 1.0 name");
		assert_eq!(entry.iso_10646_comment, "ISO 10646 comment");
		assert_eq!(entry.uppercase_mapping, CaseMapping::Some(1));
		assert_eq!(entry.lowercase_mapping, CaseMapping::Some(2));
		assert_eq!(entry.titlecase_mapping, CaseMapping::None);
	}

	#[test]
	fn supports_to_string() {
		let entry = UnicodeData {
			code: 0x12AB,
			name: "some name",
			category: Category::LetterUppercase,
			combining_class: 230,
			bidi: Bidi::L,
			decomposition: Some(Decomposition {
				tag: Some(DecompositionTag::Font),
				codes: vec![0x10, 0x20, 0x30],
			}),
			decimal_value: DecimalValue::Some(1),
			digit_value: DigitValue::Some(2),
			numeric_value: NumericValue::Rational(1, 5),
			mirrored: Mirrored::Yes,
			unicode_old_name: "old name",
			iso_10646_comment: "iso name",
			uppercase_mapping: CaseMapping::Some(0xA1),
			lowercase_mapping: CaseMapping::Some(0xB2),
			titlecase_mapping: CaseMapping::Some(0xC3),
		};
		assert_eq!(
			entry.to_string(),
			"12AB;some name;Lu;230;L;<font> 0010 0020 0030;1;2;1/5;Y;old name;iso name;00A1;00B2;00C3"
		);

		let entry = UnicodeData {
			code: 0xFF,
			name: "other name",
			category: Category::MarkEnclosing,
			combining_class: 0,
			bidi: Bidi::LRE,
			decomposition: None,
			decimal_value: DecimalValue::None,
			digit_value: DigitValue::None,
			numeric_value: NumericValue::None,
			mirrored: Mirrored::No,
			unicode_old_name: "old",
			iso_10646_comment: "",
			uppercase_mapping: CaseMapping::None,
			lowercase_mapping: CaseMapping::None,
			titlecase_mapping: CaseMapping::None,
		};
		assert_eq!(entry.to_string(), "00FF;other name;Me;0;LRE;;;;;N;old;;;;");
	}

	#[test]
	fn parses_from_string() {
		let data = UnicodeData::parse("0;name;Ll;0;L;;;;;N;;;;;").unwrap();
		assert_eq!(
			data,
			UnicodeData {
				code: 0,
				name: "name",
				category: Category::LetterLowercase,
				combining_class: 0,
				bidi: Bidi::L,
				decomposition: None,
				decimal_value: DecimalValue::None,
				digit_value: DigitValue::None,
				numeric_value: NumericValue::None,
				mirrored: Mirrored::No,
				unicode_old_name: "",
				iso_10646_comment: "",
				uppercase_mapping: CaseMapping::None,
				lowercase_mapping: CaseMapping::None,
				titlecase_mapping: CaseMapping::None,
			}
		);

		let data = UnicodeData::parse("12AB;char name;Nd;220;NSM;1234;10;20;30;Y;old;iso;AA;BB;CC")
			.unwrap();
		assert_eq!(
			data,
			UnicodeData {
				code: 0x12AB,
				name: "char name",
				category: Category::NumberDecimalDigit,
				combining_class: 220,
				bidi: Bidi::NSM,
				decomposition: Some(Decomposition {
					tag: None,
					codes: vec![0x1234]
				}),
				decimal_value: DecimalValue::Some(10),
				digit_value: DigitValue::Some(20),
				numeric_value: NumericValue::Integer(30),
				mirrored: Mirrored::Yes,
				unicode_old_name: "old",
				iso_10646_comment: "iso",
				uppercase_mapping: CaseMapping::Some(0xAA),
				lowercase_mapping: CaseMapping::Some(0xBB),
				titlecase_mapping: CaseMapping::Some(0xCC),
			}
		);
	}

	macro_rules! check_parsing {
		(input $input:literal error $error:literal) => {
			let err = UnicodeData::parse($input).expect_err(concat!(
				"expected error -- in `",
				$input,
				"`"
			));
			assert!(
				err.contains($error),
				"expected error `{}`, but it was `{}` -- in `{}`",
				$error,
				err,
				$input
			);
			assert!(
				err.contains($input),
				"expected error to contain input, but it was `{}` -- in `{}`",
				$error,
				$input
			);
			assert!(
				err.contains("parsing unicode data"),
				"expected error to `parsing unicode data`, but it was `{}` -- in `{}`",
				$error,
				$input
			);
		};
	}

	#[test]
	fn parse_panics_on_invalid_input() {
		check_parsing!(
			input ""
			error "empty input"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;N;;;0;0" // missing a field
			error "invalid row format"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;N;;;0;0;0;" // additional field
			error "invalid row format"
		);

		check_parsing!(
			input "x1;name;Ll;0;L;;0;0;0;N;;;0;0;0"
			error "`x1` is not a valid code"
		);

		check_parsing!(
			input "0;;Ll;0;L;;0;0;0;N;;;0;0;0"
			error "empty name"
		);

		check_parsing!(
			input "0;name;x2;0;L;;0;0;0;N;;;0;0;0"
			error "invalid category `x2`"
		);

		check_parsing!(
			input "0;name;Ll;x3;L;;0;0;0;N;;;0;0;0"
			error "invalid combining class `x3`"
		);

		check_parsing!(
			input "0;name;Ll;0;x4;;0;0;0;N;;;0;0;0"
			error "invalid bidirectional category `x4`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;x5;0;0;0;N;;;0;0;0"
			error "invalid decomposition `x5`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;x6;0;0;N;;;0;0;0"
			error "invalid decimal value `x6`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;x7;0;N;;;0;0;0"
			error "invalid digit value `x7`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;x8;N;;;0;0;0"
			error "invalid numeric value `x8`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;X;;;0;0;0"
			error "invalid mirrored value `X`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;N;;;x9;0;0"
			error "invalid uppercase mapping `x9`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;N;;;0;xA;0"
			error "invalid lowercase mapping `xA`"
		);

		check_parsing!(
			input "0;name;Ll;0;L;;0;0;0;N;;;0;0;xB"
			error "invalid titlecase mapping `xB`"
		);
	}

	#[test]
	fn can_load_from_ucd() {
		let source = include_ucd!("UnicodeData.txt");
		let source = source.lines().enumerate();

		let mut has_entries = false;
		for (n, input) in source {
			has_entries = true;

			let parsed = UnicodeData::parse(input).unwrap();
			let output = parsed.to_string();
			assert_eq!(
				output,
				input,
				"line {}: parsed output does not match input",
				n + 1
			);
		}

		assert!(has_entries);
	}
}
