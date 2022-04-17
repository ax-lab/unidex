/// Include an input file relative to the project root directory.
macro_rules! include_input {
	($path:expr) => {
		$crate::input::Input(include_str!(concat!("../../", $path)))
	};
}

/// Include an input file from the UCD data by the file name.
macro_rules! include_ucd {
	($filename:expr) => {
		include_input!(concat!("vendor-data/ucd/", $filename))
	};
}

/// Enum of supported input files from the UCD data.
#[derive(Clone, Copy)]
pub enum InputFile {
	Blocks,
}

/// Input wrapper providing support for reading data files from the UCD data.
pub struct Input(pub &'static str);

impl Input {
	/// Get one of the supported [`InputFile`]s from the UCD data.
	pub fn get(file: InputFile) -> Self {
		match file {
			InputFile::Blocks => include_ucd!("Blocks.txt"),
		}
	}

	/// Iterator over the input lines filtering comments and blank lines.
	pub fn lines(&self) -> impl Iterator<Item = &'static str> {
		let lines = self.0.lines();
		let lines = lines
			.map(|x| {
				if let Some(index) = x.find('#') {
					&x[0..index]
				} else {
					x
				}
			})
			.map(|x| x.trim_end())
			.filter(|x| x.len() > 0);
		lines
	}
}

#[cfg(test)]
mod tests {
	macro_rules! read_test_input {
		($filename:expr) => {
			include_input!(concat!("ucd-parser/testdata/input/", $filename))
		};
	}

	#[test]
	fn can_open_project_files_and_read_lines() {
		let input = read_test_input!("basic-123.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["line 1", "line 2", "line 3"]);

		let input = read_test_input!("basic-abc.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["line A", "line B", "line C"]);
	}

	#[test]
	fn input_lines_skip_empty() {
		let input = read_test_input!("empty-lines.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["non-empty 1", "non-empty 2", "non-empty 3"]);
	}

	#[test]
	fn input_lines_filter_out_comments() {
		let input = read_test_input!("comments.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["nc 1", "nc 2", "nc 3", "nc 4"]);
	}
}
