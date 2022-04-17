use std::path::{Path, PathBuf};

pub struct Input(String);

impl Input {
	pub fn read<T: AsRef<Path>>(filename: T) -> Self {
		let base_dir = env!("CARGO_MANIFEST_DIR");
		let filename = filename.as_ref();
		let filename = if base_dir != "" {
			let mut path = PathBuf::from(base_dir);
			path.push("../");
			path.push(filename);
			path
		} else {
			filename.to_path_buf()
		};
		let input = std::fs::read_to_string(filename).expect("reading input");
		Input(input)
	}

	pub fn lines(&self) -> impl Iterator<Item = &str> {
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
mod test_input {
	use super::Input;

	macro_rules! read_test_input {
		($filename:expr) => {
			Input::read(concat!("ucd-parser/testdata/input/", $filename))
		};
	}

	#[test]
	fn read_can_open_project_files() {
		let input = read_test_input!("basic-123.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["line 1", "line 2", "line 3"]);

		let input = read_test_input!("basic-abc.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["line A", "line B", "line C"]);
	}

	#[test]
	fn read_lines_skip_empty_lines() {
		let input = read_test_input!("empty-lines.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["non-empty 1", "non-empty 2", "non-empty 3"]);
	}

	#[test]
	fn read_lines_filter_comments() {
		let input = read_test_input!("comments.in");
		let input = input.lines().collect::<Vec<_>>();
		assert_eq!(input, vec!["nc 1", "nc 2", "nc 3", "nc 4"]);
	}
}
