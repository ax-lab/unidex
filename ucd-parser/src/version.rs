use once_cell::sync::Lazy;

use crate::input::{Input, InputFile};

pub fn unicode_version() -> &'static str {
	static VERSION: Lazy<&'static str> = Lazy::new(|| {
		let file = Input::get(InputFile::ReadMe);

		let mut file = file.text();
		let version_prefix = "Version ";
		let mut version = None;
		while let Some(index) = file.find(version_prefix) {
			file = &file[index + version_prefix.len()..];
			if let Some(char) = file.chars().next() {
				if char.is_ascii_digit() {
					let end = file
						.find(|c: char| c != '.' && !c.is_ascii_digit())
						.unwrap_or(file.len());
					version = Some(&file[..end]);
					break;
				}
			}
		}
		version.unwrap_or_default()
	});
	&VERSION
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_provide_unicode_version() {
		let version = unicode_version();
		assert!(version != "");

		let parts = version.split(".");
		let parts = parts.collect::<Vec<_>>();
		assert_eq!(parts.len(), 3);

		let parts = parts
			.into_iter()
			.map(|x| x.parse::<u32>().unwrap())
			.collect::<Vec<_>>();

		assert!(parts[0] >= 14);
	}
}
