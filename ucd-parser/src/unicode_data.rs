use super::data::*;

pub struct UnicodeData<'a> {
	code: u32,
	name: &'a str,
	category: Category,
	combining_class: u8,
	bidi: Bidi,
	decomposition: Option<Decomposition>,
	decimal_value: DecimalValue,
	digit_value: DigitValue,
	numeric_value: NumericValue,
	mirrored: Mirrored,
	unicode_old_name: &'a str,
	iso_10646_comment: &'a str,
	uppercase_mapping: CaseMapping,
	lowercase_mapping: CaseMapping,
	titlecase_mapping: CaseMapping,
}

impl<'a> UnicodeData<'a> {
	pub fn new(
		code: u32,
		name: &'a str,
		category: Category,
		combining_class: u8,
		bidi: Bidi,
		decomposition: Option<Decomposition>,
		decimal_value: DecimalValue,
		digit_value: DigitValue,
		numeric_value: NumericValue,
		mirrored: Mirrored,
		unicode_old_name: &'a str,
		iso_10646_comment: &'a str,
		uppercase_mapping: CaseMapping,
		lowercase_mapping: CaseMapping,
		titlecase_mapping: CaseMapping,
	) -> Self {
		UnicodeData {
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
		}
	}

	/// Codepoint value. Note that for codepoint ranges this can represent the
	/// start or end of a range of codepoints.
	pub fn code(&self) -> u32 {
		self.code
	}

	/// Character name.
	pub fn name(&self) -> &'a str {
		self.name
	}

	/// General category for the character.
	pub fn category(&self) -> Category {
		self.category
	}

	/// The classes used for the Canonical Ordering Algorithm in the Unicode
	/// standard.
	pub fn combining_class(&self) -> u8 {
		self.combining_class
	}

	/// Bidirectional category for this character.
	pub fn bidi(&self) -> Bidi {
		self.bidi
	}

	/// Decomposition mapping for the character.
	pub fn decomposition(&self) -> &Option<Decomposition> {
		&self.decomposition
	}

	/// Decimal digit value, if the character has the decimal digit property.
	pub fn decimal_value(&self) -> DecimalValue {
		self.decimal_value
	}

	/// Digit value, if the character represents a digit, not necessarily a
	/// decimal digit.
	pub fn digit_value(&self) -> DigitValue {
		self.digit_value
	}

	/// Numeric value, if the character has the numeric property.
	pub fn numeric_value(&self) -> NumericValue {
		self.numeric_value
	}

	/// If the character has been identified as a "mirrored" character in
	/// bidirectional text this is [`Mirrored::Yes`].
	pub fn mirrored(&self) -> Mirrored {
		self.mirrored
	}

	/// Old name as published in Unicode 1.0. This name is only provided
	/// when it is significantly different from the Unicode 3.0 name for
	/// the character.
	pub fn unicode_old_name(&self) -> &'a str {
		self.unicode_old_name
	}

	/// This is the ISO 10646 comment field. It is in parantheses in the 10646
	/// names list.
	pub fn iso_10646_comment(&self) -> &'a str {
		self.iso_10646_comment
	}

	/// Uppercase mapping for this character.
	pub fn uppercase_mapping(&self) -> CaseMapping {
		self.uppercase_mapping
	}

	/// Lowercase mapping for this character.
	pub fn lowercase_mapping(&self) -> CaseMapping {
		self.lowercase_mapping
	}

	/// Titlecase mapping for this character.
	pub fn titlecase_mapping(&self) -> CaseMapping {
		self.titlecase_mapping
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
		let entry = UnicodeData::new(
			0x12AB,
			"char name",
			Category::LetterLowercase,
			230,
			Bidi::L,
			Some(decomposition.clone()),
			DecimalValue::Some(1),
			DigitValue::Some(1),
			NumericValue::Rational(1, 5),
			Mirrored::Yes,
			"unicode 1.0 name",
			"ISO 10646 comment",
			CaseMapping::Some(1),
			CaseMapping::Some(2),
			CaseMapping::None,
		);
		assert_eq!(entry.code(), 0x12AB);
		assert_eq!(entry.name(), "char name");
		assert_eq!(entry.category(), Category::LetterLowercase);
		assert_eq!(entry.combining_class(), 230);
		assert_eq!(entry.bidi(), Bidi::L);
		assert_eq!(entry.decomposition(), &Some(decomposition));
		assert_eq!(entry.decimal_value(), DecimalValue::Some(1));
		assert_eq!(entry.digit_value(), DigitValue::Some(1));
		assert_eq!(entry.numeric_value(), NumericValue::Rational(1, 5));
		assert_eq!(entry.mirrored(), Mirrored::Yes);
		assert_eq!(entry.unicode_old_name(), "unicode 1.0 name");
		assert_eq!(entry.iso_10646_comment(), "ISO 10646 comment");
		assert_eq!(entry.uppercase_mapping(), CaseMapping::Some(1));
		assert_eq!(entry.lowercase_mapping(), CaseMapping::Some(2));
		assert_eq!(entry.titlecase_mapping(), CaseMapping::None);
	}

	#[test]
	fn supports_to_string() {
		let entry = UnicodeData::new(
			0x12AB,
			"some name",
			Category::LetterUppercase,
			230,
			Bidi::L,
			Some(Decomposition {
				tag: Some(DecompositionTag::Font),
				codes: vec![0x10, 0x20, 0x30],
			}),
			DecimalValue::Some(1),
			DigitValue::Some(2),
			NumericValue::Rational(1, 5),
			Mirrored::Yes,
			"old name",
			"iso name",
			CaseMapping::Some(0xA1),
			CaseMapping::Some(0xB2),
			CaseMapping::Some(0xC3),
		);
		assert_eq!(
			entry.to_string(),
			"12AB;some name;Lu;230;L;<font> 0010 0020 0030;1;2;1/5;Y;old name;iso name;00A1;00B2;00C3"
		);

		let entry = UnicodeData::new(
			0xFF,
			"other name",
			Category::MarkEnclosing,
			0,
			Bidi::LRE,
			None,
			DecimalValue::None,
			DigitValue::None,
			NumericValue::None,
			Mirrored::No,
			"old",
			"",
			CaseMapping::None,
			CaseMapping::None,
			CaseMapping::None,
		);
		assert_eq!(entry.to_string(), "00FF;other name;Me;0;LRE;;;;;N;old;;;;");
	}
}
