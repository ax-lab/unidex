use std::ops::RangeInclusive;

use once_cell::sync::Lazy;

use crate::{
	input::{Input, InputFile},
	parse::parse_range,
};

/// Block of codepoints from the Unicode Character Database.
///
/// ```
/// # use ucd_parser::blocks::Block;
/// let blocks = Block::list();
/// for block in blocks {
///     println!("{}: {:?}", block.name, block.range);
/// }
/// ```
#[derive(Debug)]
pub struct Block<'a> {
	/// Inclusive range of codepoints in this block.
	pub range: RangeInclusive<u32>,

	/// Name for the block.
	///
	/// When comparing block names, casing, whitespace, hyphens, and underbars
	/// are ignored. For example, "Latin Extended-A" and "latin extended a" are
	/// equivalent.
	pub name: &'a str,
}

impl<'a> Block<'a> {
	/// List of blocks from the UCD data. Lazy-loaded from `Blocks.txt`.
	pub fn list() -> &'static [Block<'static>] {
		static BLOCKS: Lazy<Box<[Block]>> = Lazy::new(|| {
			let input = Input::get(InputFile::Blocks);
			let lines = input.lines();
			let blocks = lines.map(|x| Block::parse(x).unwrap());
			let blocks = blocks.collect::<Vec<_>>();
			blocks.into_boxed_slice()
		});
		&BLOCKS
	}

	pub fn new(range: RangeInclusive<u32>, name: &'a str) -> Self {
		Block { range, name }
	}

	pub fn parse(input: &'a str) -> Result<Self, String> {
		let semicolon = input
			.find(";")
			.ok_or_else(|| format!("`{}` block is missing `;`", input))?;
		let (range, name) = (&input[..semicolon], &input[semicolon + 1..].trim());
		let (start, end) =
			parse_range(range).map_err(|err| format!("block {} -- in `{}`", err, input))?;
		Ok(Block::new(start..=end, name))
	}
}

impl<'a> std::fmt::Display for Block<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:04X}..{:04X}; {}",
			self.range.start(),
			self.range.end(),
			self.name
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn supports_to_string() {
		let block = Block::new(1..=255, "test block");
		assert_eq!(block.to_string(), "0001..00FF; test block");

		let block = Block::new(0..=0xFCFC, "other block");
		assert_eq!(block.to_string(), "0000..FCFC; other block");
	}

	#[test]
	fn supports_parsing_from_string() {
		let input = "0001..00FF; test block";
		let block = Block::parse(input).unwrap();
		assert_eq!(block.range, 1..=255);
		assert_eq!(block.name, "test block");

		let input = "0000..FCFC; other block";
		let block = Block::parse(input).unwrap();
		assert_eq!(block.range, 0..=0xFCFC);
		assert_eq!(block.name, "other block");
	}

	#[test]
	fn parsing_invalid_block_returns_error() {
		let input = "xx";
		let error = Block::parse(input).unwrap_err();
		assert!(error.contains("`xx` block is missing `;`"));

		let input = "xx..00FF; some name";
		let error = Block::parse(input).unwrap_err();
		assert!(error.contains("block range start `xx` is not a valid code"));
		assert!(error.contains("-- in `xx..00FF; some name`"))
	}

	#[test]
	fn can_load_from_ucd() {
		let source = include_ucd!("Blocks.txt");
		let source = source.lines().collect::<Vec<_>>();
		assert!(source.len() > 0);

		let blocks = Block::list();
		let blocks = blocks.iter().map(|x| x.to_string()).collect::<Vec<_>>();
		assert_eq!(blocks, source);
	}
}
