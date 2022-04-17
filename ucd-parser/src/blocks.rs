use std::ops::RangeInclusive;

use once_cell::sync::Lazy;

use crate::input::{Input, InputFile};

pub struct Block<'a> {
	range: RangeInclusive<u32>,
	name: &'a str,
}

impl<'a> Block<'a> {
	pub fn list() -> &'static [Block<'static>] {
		static BLOCKS: Lazy<Box<[Block]>> = Lazy::new(|| {
			let input = Input::get(InputFile::Blocks);
			let lines = input.lines();
			let blocks = lines.map(|x| Block::parse(x));
			let blocks = blocks.collect::<Vec<_>>();
			blocks.into_boxed_slice()
		});
		&BLOCKS
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

	pub fn parse(s: &'a str) -> Self {
		let semicolon = s.find(";").expect("parsing block: missing `;`");
		let (range, name) = (&s[..semicolon], &s[semicolon + 1..].trim());
		let (start, end) = parse_range!(range, "block: range");
		Block::new(start..=end, name)
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
		let block = Block::parse(input);
		assert_eq!(block.range(), 1..=255);
		assert_eq!(block.name(), "test block");

		let input = "0000..FCFC; other block";
		let block = Block::parse(input);
		assert_eq!(block.range(), 0..=0xFCFC);
		assert_eq!(block.name(), "other block");
	}

	#[test]
	fn can_load_file() {
		let source = include_ucd!("Blocks.txt");
		let source = source.lines().collect::<Vec<_>>();

		let blocks = Block::list();
		let blocks = blocks.iter().map(|x| x.to_string()).collect::<Vec<_>>();
		assert_eq!(blocks, source);
	}
}
