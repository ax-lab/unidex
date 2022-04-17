use std::ops::RangeInclusive;

use crate::Input;

#[derive(Debug)]
pub enum ParseError {
	BlockFormat,
	BlockRangeStart,
	BlockRangeEnd,
}

#[allow(unused)]
pub struct Block<'a> {
	range: RangeInclusive<u32>,
	name: &'a str,
}

impl<'a> Block<'a> {
	pub fn load() -> impl Iterator<Item = Block<'static>> {
		let input = Input::read("vendor-data/ucd/Blocks.txt");
		let lines = input.lines();
		lines.map(|x| Block::parse(x).unwrap())
	}

	pub fn new(range: RangeInclusive<u32>, name: &'a str) -> Self {
		Block { range, name }
	}

	pub fn range(&self) -> RangeInclusive<u32> {
		self.range.clone()
	}

	pub fn name(&self) -> &str {
		self.name
	}

	pub fn parse(s: &'a str) -> Result<Self, ParseError> {
		let semicolon = s.find(";").ok_or(ParseError::BlockFormat)?;
		let name = &s[semicolon + 1..].trim();
		let range = &s[..semicolon];
		let ellipsis = range.find("..").ok_or(ParseError::BlockFormat)?;
		let start =
			u32::from_str_radix(&range[..ellipsis], 16).map_err(|_| ParseError::BlockRangeStart)?;
		let end = u32::from_str_radix(&range[ellipsis + 2..], 16)
			.map_err(|_| ParseError::BlockRangeEnd)?;
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
	use super::Block;

	#[test]
	fn block_supports_to_string() {
		let block = Block::new(1..=255, "test block");
		assert_eq!(block.to_string(), "0001..00FF; test block");

		let block = Block::new(0..=0xFCFC, "other block");
		assert_eq!(block.to_string(), "0000..FCFC; other block");
	}

	#[test]
	fn block_supports_parsing() {
		let input = "0001..00FF; test block";
		let block = Block::parse(input).unwrap();
		assert_eq!(block.range(), 1..=255);
		assert_eq!(block.name(), "test block");

		let input = "0000..FCFC; other block";
		let block = Block::parse(input).unwrap();
		assert_eq!(block.range(), 0..=0xFCFC);
		assert_eq!(block.name(), "other block");
	}

	#[test]
	fn can_load_file() {
		let source = crate::Input::read("vendor-data/ucd/Blocks.txt");
		let source = source.lines().collect::<Vec<_>>();

		let blocks = Block::load();
		let blocks = blocks.map(|x| x.to_string()).collect::<Vec<_>>();
		assert_eq!(blocks, source);
	}
}
