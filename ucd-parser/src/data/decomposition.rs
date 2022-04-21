/// Decomposition mapping for the character.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposition {
	pub tag: Option<DecompositionTag>,
	pub codes: Vec<u32>,
}

impl Decomposition {
	pub fn parse<T: AsRef<str>>(input: T, message: &str) -> Option<Self> {
		let input = input.as_ref();
		if input.len() == 0 {
			return None;
		}

		let mut output = Decomposition {
			tag: None,
			codes: Default::default(),
		};
		for (n, value) in input.split(' ').enumerate() {
			if n == 0 && value.starts_with('<') {
				output.tag = DecompositionTag::parse(value);
				output
					.tag
					.ok_or_else(|| {
						format!(
							"parsing {}: decomposition tag `{}` is not valid",
							message, value
						)
					})
					.unwrap();
			} else {
				let code = u32::from_str_radix(value, 16)
					.map_err(|err| {
						format!(
							"parsing {}: decomposition code `{}` is not valid -- {}",
							message, value, err
						)
					})
					.unwrap();
				output.codes.push(code);
			}
		}
		Some(output)
	}
}

/// The tags supplied with certain [`Decomposition`] mappings generally indicate
/// formatting information.
///
/// Where no such tag is given, the mapping is designated as canonical.
///
/// Conversely, the presence of a formatting tag also indicates that the
/// mapping is a compatibility mapping and not a canonical mapping.
///
/// In the absence of other formatting information in a compatibility mapping,
/// the tag is used to distinguish it from canonical mappings.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecompositionTag {
	/// A font variant (e.g. a blackletter form).
	Font,
	/// A no-break version of a space or hyphen.
	NoBreak,
	/// An initial presentation form (Arabic).
	Initial,
	/// A medial presentation form (Arabic).
	Medial,
	/// A final presentation form (Arabic).
	Final,
	/// An isolated presentation form (Arabic).
	Isolated,
	/// An encircled form.
	Circle,
	/// A superscript form.
	Super,
	/// A subscript form.
	Sub,
	/// A vertical layout presentation form.
	Vertical,
	/// A wide (or zenkaku) compatibility character.
	Wide,
	/// A narrow (or hankaku) compatibility character.
	Narrow,
	/// A small variant form (CNS compatibility).
	Small,
	/// A CJK squared font variant.
	Square,
	/// A vulgar fraction form.
	Fraction,
	/// Otherwise unspecified compatibility character.
	Compat,
}

impl DecompositionTag {
	pub fn parse<T: AsRef<str>>(input: T) -> Option<Self> {
		let tag = match input.as_ref() {
			"<font>" => DecompositionTag::Font,
			"<noBreak>" => DecompositionTag::NoBreak,
			"<initial>" => DecompositionTag::Initial,
			"<medial>" => DecompositionTag::Medial,
			"<final>" => DecompositionTag::Final,
			"<isolated>" => DecompositionTag::Isolated,
			"<circle>" => DecompositionTag::Circle,
			"<super>" => DecompositionTag::Super,
			"<sub>" => DecompositionTag::Sub,
			"<vertical>" => DecompositionTag::Vertical,
			"<wide>" => DecompositionTag::Wide,
			"<narrow>" => DecompositionTag::Narrow,
			"<small>" => DecompositionTag::Small,
			"<square>" => DecompositionTag::Square,
			"<fraction>" => DecompositionTag::Fraction,
			"<compat>" => DecompositionTag::Compat,
			_ => return None,
		};
		Some(tag)
	}
}

#[cfg(test)]
mod test_decomposition {
	use super::*;

	#[test]
	fn parses_empty_string_as_none() {
		assert_eq!(Decomposition::parse("", "some row"), None);
	}

	#[test]
	fn parses_without_a_tag() {
		let input = "309D 3099";
		assert_eq!(
			Decomposition::parse(input, "some row").unwrap(),
			Decomposition {
				tag: None,
				codes: vec![0x309D, 0x3099]
			}
		);
	}

	#[test]
	fn parses_with_a_tag() {
		let input = "<vertical> 3088 308A";
		assert_eq!(
			Decomposition::parse(input, "some row").unwrap(),
			Decomposition {
				tag: Some(DecompositionTag::Vertical),
				codes: vec![0x3088, 0x308A]
			}
		);

		let input = "<compat> 1100";
		assert_eq!(
			Decomposition::parse(input, "some row").unwrap(),
			Decomposition {
				tag: Some(DecompositionTag::Compat),
				codes: vec![0x1100]
			}
		);
	}

	#[test]
	#[should_panic = "parsing some row: decomposition tag `<xx>` is not valid"]
	fn panics_on_invalid_tag() {
		let input = "<xx> FFFF";
		Decomposition::parse(input, "some row");
	}

	#[test]
	#[should_panic = "parsing some row: decomposition code `XX` is not valid"]
	fn panics_on_invalid_code() {
		let input = "FFFF XX FFFF";
		Decomposition::parse(input, "some row");
	}
}

#[cfg(test)]
mod test_tags {
	use super::*;

	#[test]
	fn parses_tag_from_string() {
		fn parse(input: &'static str) -> DecompositionTag {
			DecompositionTag::parse(input).unwrap()
		}

		assert_eq!(parse("<font>"), DecompositionTag::Font);
		assert_eq!(parse("<noBreak>"), DecompositionTag::NoBreak);
		assert_eq!(parse("<initial>"), DecompositionTag::Initial);
		assert_eq!(parse("<medial>"), DecompositionTag::Medial);
		assert_eq!(parse("<final>"), DecompositionTag::Final);
		assert_eq!(parse("<isolated>"), DecompositionTag::Isolated);
		assert_eq!(parse("<circle>"), DecompositionTag::Circle);
		assert_eq!(parse("<super>"), DecompositionTag::Super);
		assert_eq!(parse("<sub>"), DecompositionTag::Sub);
		assert_eq!(parse("<vertical>"), DecompositionTag::Vertical);
		assert_eq!(parse("<wide>"), DecompositionTag::Wide);
		assert_eq!(parse("<narrow>"), DecompositionTag::Narrow);
		assert_eq!(parse("<small>"), DecompositionTag::Small);
		assert_eq!(parse("<square>"), DecompositionTag::Square);
		assert_eq!(parse("<fraction>"), DecompositionTag::Fraction);
		assert_eq!(parse("<compat>"), DecompositionTag::Compat);
	}

	#[test]
	fn parse_tag_from_invalid_string_is_none() {
		assert_eq!(DecompositionTag::parse("xx"), None);
	}
}
