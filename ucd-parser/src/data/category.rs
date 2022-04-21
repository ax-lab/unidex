/// General category for character. These are a useful breakdown into
/// various "character types" which can be used as a default categorization
/// in implementations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Category {
	/// `Cn` Unicode category (no characters in the file have this property).
	OtherNotAssigned,
	/// `Lu` Unicode category.
	LetterUppercase,
	/// `Ll` Unicode category.
	LetterLowercase,
	/// `Lt` Unicode category.
	LetterTitlecase,
	/// `Mn` Unicode category.
	MarkNonSpacing,
	/// `Mc` Unicode category.
	MarkSpacingCombining,
	/// `Me` Unicode category.
	MarkEnclosing,
	/// `Nd` Unicode category.
	NumberDecimalDigit,
	/// `Nl` Unicode category.
	NumberLetter,
	/// `No` Unicode category.
	NumberOther,
	/// `Zs` Unicode category.
	SeparatorSpace,
	/// `Zl` Unicode category.
	SeparatorLine,
	/// `Zp` Unicode category.
	SeparatorParagraph,
	/// `Cc` Unicode category.
	OtherControl,
	/// `Cf` Unicode category.
	OtherFormat,
	/// `Cs` Unicode category.
	OtherSurrogate,
	/// `Co` Unicode category.
	OtherPrivateUse,
	/// `Lm` Unicode category.
	LetterModifier,
	/// `Lo` Unicode category.
	LetterOther,
	/// `Pc` Unicode category.
	PunctuationConnector,
	/// `Pd` Unicode category.
	PunctuationDash,
	/// `Ps` Unicode category.
	PunctuationOpen,
	/// `Pe` Unicode category.
	PunctuationClose,
	/// `Pi` Unicode category (may behave like Ps or Pe depending on usage).
	PunctuationInitialQuote,
	/// `Pf` Unicode category (may behave like Ps or Pe depending on usage).
	PunctuationFinalQuote,
	/// `Po` Unicode category.
	PunctuationOther,
	/// `Sm` Unicode category.
	SymbolMath,
	/// `Sc` Unicode category.
	SymbolCurrency,
	/// `Sk` Unicode category.
	SymbolModifier,
	/// `So` Unicode category.
	SymbolOther,
}

impl Category {
	pub fn parse<T: AsRef<str>>(input: T) -> Option<Self> {
		let category = match input.as_ref() {
			"Cn" => Category::OtherNotAssigned,
			"Lu" => Category::LetterUppercase,
			"Ll" => Category::LetterLowercase,
			"Lt" => Category::LetterTitlecase,
			"Mn" => Category::MarkNonSpacing,
			"Mc" => Category::MarkSpacingCombining,
			"Me" => Category::MarkEnclosing,
			"Nd" => Category::NumberDecimalDigit,
			"Nl" => Category::NumberLetter,
			"No" => Category::NumberOther,
			"Zs" => Category::SeparatorSpace,
			"Zl" => Category::SeparatorLine,
			"Zp" => Category::SeparatorParagraph,
			"Cc" => Category::OtherControl,
			"Cf" => Category::OtherFormat,
			"Cs" => Category::OtherSurrogate,
			"Co" => Category::OtherPrivateUse,
			"Lm" => Category::LetterModifier,
			"Lo" => Category::LetterOther,
			"Pc" => Category::PunctuationConnector,
			"Pd" => Category::PunctuationDash,
			"Ps" => Category::PunctuationOpen,
			"Pe" => Category::PunctuationClose,
			"Pi" => Category::PunctuationInitialQuote,
			"Pf" => Category::PunctuationFinalQuote,
			"Po" => Category::PunctuationOther,
			"Sm" => Category::SymbolMath,
			"Sc" => Category::SymbolCurrency,
			"Sk" => Category::SymbolModifier,
			"So" => Category::SymbolOther,
			_ => return None,
		};
		Some(category)
	}
}

impl std::fmt::Display for Category {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output = match self {
			Category::OtherNotAssigned => "Cn",
			Category::LetterUppercase => "Lu",
			Category::LetterLowercase => "Ll",
			Category::LetterTitlecase => "Lt",
			Category::MarkNonSpacing => "Mn",
			Category::MarkSpacingCombining => "Mc",
			Category::MarkEnclosing => "Me",
			Category::NumberDecimalDigit => "Nd",
			Category::NumberLetter => "Nl",
			Category::NumberOther => "No",
			Category::SeparatorSpace => "Zs",
			Category::SeparatorLine => "Zl",
			Category::SeparatorParagraph => "Zp",
			Category::OtherControl => "Cc",
			Category::OtherFormat => "Cf",
			Category::OtherSurrogate => "Cs",
			Category::OtherPrivateUse => "Co",
			Category::LetterModifier => "Lm",
			Category::LetterOther => "Lo",
			Category::PunctuationConnector => "Pc",
			Category::PunctuationDash => "Pd",
			Category::PunctuationOpen => "Ps",
			Category::PunctuationClose => "Pe",
			Category::PunctuationInitialQuote => "Pi",
			Category::PunctuationFinalQuote => "Pf",
			Category::PunctuationOther => "Po",
			Category::SymbolMath => "Sm",
			Category::SymbolCurrency => "Sc",
			Category::SymbolModifier => "Sk",
			Category::SymbolOther => "So",
		};
		write!(f, "{}", output)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_from_string() {
		fn parse(input: &'static str) -> Category {
			Category::parse(input).unwrap()
		}

		assert_eq!(parse("Cn"), Category::OtherNotAssigned);
		assert_eq!(parse("Lu"), Category::LetterUppercase);
		assert_eq!(parse("Ll"), Category::LetterLowercase);
		assert_eq!(parse("Lt"), Category::LetterTitlecase);
		assert_eq!(parse("Mn"), Category::MarkNonSpacing);
		assert_eq!(parse("Mc"), Category::MarkSpacingCombining);
		assert_eq!(parse("Me"), Category::MarkEnclosing);
		assert_eq!(parse("Nd"), Category::NumberDecimalDigit);
		assert_eq!(parse("Nl"), Category::NumberLetter);
		assert_eq!(parse("No"), Category::NumberOther);
		assert_eq!(parse("Zs"), Category::SeparatorSpace);
		assert_eq!(parse("Zl"), Category::SeparatorLine);
		assert_eq!(parse("Zp"), Category::SeparatorParagraph);
		assert_eq!(parse("Cc"), Category::OtherControl);
		assert_eq!(parse("Cf"), Category::OtherFormat);
		assert_eq!(parse("Cs"), Category::OtherSurrogate);
		assert_eq!(parse("Co"), Category::OtherPrivateUse);
		assert_eq!(parse("Lm"), Category::LetterModifier);
		assert_eq!(parse("Lo"), Category::LetterOther);
		assert_eq!(parse("Pc"), Category::PunctuationConnector);
		assert_eq!(parse("Pd"), Category::PunctuationDash);
		assert_eq!(parse("Ps"), Category::PunctuationOpen);
		assert_eq!(parse("Pe"), Category::PunctuationClose);
		assert_eq!(parse("Pi"), Category::PunctuationInitialQuote);
		assert_eq!(parse("Pf"), Category::PunctuationFinalQuote);
		assert_eq!(parse("Po"), Category::PunctuationOther);
		assert_eq!(parse("Sm"), Category::SymbolMath);
		assert_eq!(parse("Sc"), Category::SymbolCurrency);
		assert_eq!(parse("Sk"), Category::SymbolModifier);
		assert_eq!(parse("So"), Category::SymbolOther);
	}

	#[test]
	fn parse_from_invalid_string_is_none() {
		assert_eq!(Category::parse("xx"), None);
	}

	#[test]
	fn supports_to_string() {
		fn check(input: Category, expected: &'static str) {
			assert_eq!(input.to_string(), expected);
			assert_eq!(Category::parse(input.to_string()).expect(expected), input);
		}

		check(Category::OtherNotAssigned, "Cn");
		check(Category::LetterUppercase, "Lu");
		check(Category::LetterLowercase, "Ll");
		check(Category::LetterTitlecase, "Lt");
		check(Category::MarkNonSpacing, "Mn");
		check(Category::MarkSpacingCombining, "Mc");
		check(Category::MarkEnclosing, "Me");
		check(Category::NumberDecimalDigit, "Nd");
		check(Category::NumberLetter, "Nl");
		check(Category::NumberOther, "No");
		check(Category::SeparatorSpace, "Zs");
		check(Category::SeparatorLine, "Zl");
		check(Category::SeparatorParagraph, "Zp");
		check(Category::OtherControl, "Cc");
		check(Category::OtherFormat, "Cf");
		check(Category::OtherSurrogate, "Cs");
		check(Category::OtherPrivateUse, "Co");
		check(Category::LetterModifier, "Lm");
		check(Category::LetterOther, "Lo");
		check(Category::PunctuationConnector, "Pc");
		check(Category::PunctuationDash, "Pd");
		check(Category::PunctuationOpen, "Ps");
		check(Category::PunctuationClose, "Pe");
		check(Category::PunctuationInitialQuote, "Pi");
		check(Category::PunctuationFinalQuote, "Pf");
		check(Category::PunctuationOther, "Po");
		check(Category::SymbolMath, "Sm");
		check(Category::SymbolCurrency, "Sc");
		check(Category::SymbolModifier, "Sk");
		check(Category::SymbolOther, "So");
	}
}
