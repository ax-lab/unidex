/// Decomposition mapping for the character.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposition {
	pub tag: Option<DecompositionTag>,
	pub codes: Vec<u32>,
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
	Font,
}
