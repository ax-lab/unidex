use super::data::*;

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
	pub combining_class: u8,

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
}
