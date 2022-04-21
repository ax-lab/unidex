/// These are the categories required by the Bidirectional Behavior Algorithm
/// in the Unicode Standard.
///
/// See https://www.unicode.org/reports/tr9/#Bidirectional_Character_Types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Bidi {
	/// Left-to-Right: most alphabetic, syllabic, Han ideographs, non-European
	/// or non-Arabic digits, ...
	L,
	/// Right-to-Left: RLM, Hebrew alphabet, and related punctuation
	R,
	/// Right-to-Left Arabic: ALM, Arabic, Thaana, and Syriac alphabets, most
	/// punctuation specific to those scripts, ...
	AL,
	/// European Number: European digits, Eastern Arabic-Indic digits, ...
	EN,
	/// European Number Separator: PLUS SIGN, MINUS SIGN
	ES,
	/// European Number Terminator: DEGREE SIGN, currency symbols, ...
	ET,
	/// Arabic Number: Arabic-Indic digits, Arabic decimal and thousands
	/// separators, ...
	AN,
	/// Common Number Separator: COLON, COMMA, FULL STOP, NO-BREAK SPACE, ...
	CS,
	/// Nonspacing Mark: Characters with the General Category values: Mn
	/// (Nonspacing_Mark) and Me (Enclosing_Mark)
	NSM,
	/// Boundary Neutral: Default ignorables, non-characters, and control
	/// characters, other than those explicitly given other types.
	BN,
	/// Paragraph Separator: PARAGRAPH SEPARATOR, appropriate Newline Functions,
	/// higher-level protocol paragraph determination
	B,
	/// Segment Separator: Tab
	S,
	/// Whitespace: SPACE, FIGURE SPACE, LINE SEPARATOR, FORM FEED, general
	/// punctuation spaces, ...
	WS,
	/// Other Neutrals: All other characters, including OBJECT REPLACEMENT CHARACTER
	ON,
	/// Left-to-Right Embedding
	LRE,
	/// Left-to-Right Override
	LRO,
	/// Right-to-Left Embedding
	RLE,
	/// Right-to-Left Override
	RLO,
	/// Pop Directional Format
	PDF,
	/// Left-to-Right Isolate
	LRI,
	/// Right-to-Left Isolate
	RLI,
	/// First Strong Isolate
	FSI,
	/// Pop Directional Isolate
	PDI,
}

impl Bidi {
	pub fn parse<T: AsRef<str>>(input: T) -> Option<Bidi> {
		let bidi = match input.as_ref() {
			"L" => Bidi::L,
			"R" => Bidi::R,
			"AL" => Bidi::AL,
			"EN" => Bidi::EN,
			"ES" => Bidi::ES,
			"ET" => Bidi::ET,
			"AN" => Bidi::AN,
			"CS" => Bidi::CS,
			"NSM" => Bidi::NSM,
			"BN" => Bidi::BN,
			"B" => Bidi::B,
			"S" => Bidi::S,
			"WS" => Bidi::WS,
			"ON" => Bidi::ON,
			"LRE" => Bidi::LRE,
			"LRO" => Bidi::LRO,
			"RLE" => Bidi::RLE,
			"RLO" => Bidi::RLO,
			"PDF" => Bidi::PDF,
			"LRI" => Bidi::LRI,
			"RLI" => Bidi::RLI,
			"FSI" => Bidi::FSI,
			"PDI" => Bidi::PDI,
			_ => return None,
		};
		Some(bidi)
	}
}

impl std::fmt::Display for Bidi {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output = match self {
			Bidi::L => "L",
			Bidi::R => "R",
			Bidi::AL => "AL",
			Bidi::EN => "EN",
			Bidi::ES => "ES",
			Bidi::ET => "ET",
			Bidi::AN => "AN",
			Bidi::CS => "CS",
			Bidi::NSM => "NSM",
			Bidi::BN => "BN",
			Bidi::B => "B",
			Bidi::S => "S",
			Bidi::WS => "WS",
			Bidi::ON => "ON",
			Bidi::LRE => "LRE",
			Bidi::LRO => "LRO",
			Bidi::RLE => "RLE",
			Bidi::RLO => "RLO",
			Bidi::PDF => "PDF",
			Bidi::LRI => "LRI",
			Bidi::RLI => "RLI",
			Bidi::FSI => "FSI",
			Bidi::PDI => "PDI",
		};
		write!(f, "{}", output)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_from_string() {
		fn parse(input: &'static str) -> Bidi {
			Bidi::parse(input).unwrap()
		}

		assert_eq!(parse("L"), Bidi::L);
		assert_eq!(parse("R"), Bidi::R);
		assert_eq!(parse("AL"), Bidi::AL);
		assert_eq!(parse("EN"), Bidi::EN);
		assert_eq!(parse("ES"), Bidi::ES);
		assert_eq!(parse("ET"), Bidi::ET);
		assert_eq!(parse("AN"), Bidi::AN);
		assert_eq!(parse("CS"), Bidi::CS);
		assert_eq!(parse("NSM"), Bidi::NSM);
		assert_eq!(parse("BN"), Bidi::BN);
		assert_eq!(parse("B"), Bidi::B);
		assert_eq!(parse("S"), Bidi::S);
		assert_eq!(parse("WS"), Bidi::WS);
		assert_eq!(parse("ON"), Bidi::ON);
		assert_eq!(parse("LRE"), Bidi::LRE);
		assert_eq!(parse("LRO"), Bidi::LRO);
		assert_eq!(parse("RLE"), Bidi::RLE);
		assert_eq!(parse("RLO"), Bidi::RLO);
		assert_eq!(parse("PDF"), Bidi::PDF);
		assert_eq!(parse("LRI"), Bidi::LRI);
		assert_eq!(parse("RLI"), Bidi::RLI);
		assert_eq!(parse("FSI"), Bidi::FSI);
		assert_eq!(parse("PDI"), Bidi::PDI);
	}

	#[test]
	fn parse_from_invalid_string_is_none() {
		assert_eq!(Bidi::parse("xx"), None);
	}

	#[test]
	fn supports_to_string() {
		fn check(input: Bidi, expected: &'static str) {
			assert_eq!(input.to_string(), expected);
			assert_eq!(Bidi::parse(input.to_string()).expect(expected), input);
		}

		check(Bidi::L, "L");
		check(Bidi::R, "R");
		check(Bidi::AL, "AL");
		check(Bidi::EN, "EN");
		check(Bidi::ES, "ES");
		check(Bidi::ET, "ET");
		check(Bidi::AN, "AN");
		check(Bidi::CS, "CS");
		check(Bidi::NSM, "NSM");
		check(Bidi::BN, "BN");
		check(Bidi::B, "B");
		check(Bidi::S, "S");
		check(Bidi::WS, "WS");
		check(Bidi::ON, "ON");
		check(Bidi::LRE, "LRE");
		check(Bidi::LRO, "LRO");
		check(Bidi::RLE, "RLE");
		check(Bidi::RLO, "RLO");
		check(Bidi::PDF, "PDF");
		check(Bidi::LRI, "LRI");
		check(Bidi::RLI, "RLI");
		check(Bidi::FSI, "FSI");
		check(Bidi::PDI, "PDI");
	}
}
