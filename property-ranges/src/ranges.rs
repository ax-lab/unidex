/// Single range in a [`CodepointRangeMap`].
#[derive(Clone, Eq, PartialEq)]
pub struct CodepointRange<T> {
	pub first: u32,
	pub last: u32,
	pub value: T,
}

/// Map ranges of [`u32`] codepoints to their respective values.
///
/// This map supports building a sorted list of codepoint ranges mapping to
/// specific values. As range values are set and updated, this provides the
/// necessary logic to split input ranges into sub-ranges according to their
/// unique values.
///
/// To set the value for a range call [`set`](CodepointRangeMap::set) with an
/// updater function. The map splits overlapping ranges and calls the function
/// to set or update the value for each sub-range:
///
/// ```
/// # use property_ranges::*;
/// let mut map = CodepointRangeMap::default();
/// map.set(0, 5, |v| *v = 100); // `v` starts as zero
/// map.set(3, 9, |v| *v += 25); // `v` is 100 for the existing `3..=5` range
/// assert!(map.count() == 3);
/// assert!(map.get(0) == &CodepointRange{ first: 0, last: 2, value: 100 });
/// assert!(map.get(1) == &CodepointRange{ first: 3, last: 5, value: 125 });
/// assert!(map.get(2) == &CodepointRange{ first: 6, last: 9, value: 25  });
/// ```
pub struct CodepointRangeMap<T: Default + Clone> {
	ranges: Vec<CodepointRange<T>>,
}

impl<T: Default + Clone> CodepointRangeMap<T> {
	pub fn count(&self) -> usize {
		self.ranges.len()
	}

	/// Set the value for an inclusive range using an updater function.
	///
	/// If the input range overlaps existing ranges, this will split the input
	/// range accordingly and call the updater function for each sub-range.
	///
	/// For each sub-range, the updater will receive a mutable reference for
	/// the current value of that sub-range. If the sub-range is not yet mapped,
	/// then the reference value will be the default.
	pub fn set<Fn: FnMut(&mut T)>(&mut self, mut first: u32, last: u32, mut updater: Fn) {
		if last < first {
			panic!("CodepointRangeMap: invalid range (last < first)");
		}

		// this is very un-optimized, but we are only supposed to be used
		// during code generation, so unless build times become an issue we
		// should be okay
		let mut new_entries = Vec::new();
		for range in self.ranges.iter_mut() {
			if first < range.first {
				let mut value = T::default();
				updater(&mut value);

				let last = std::cmp::min(range.first - 1, last);
				new_entries.push(CodepointRange { first, last, value });
				first = range.first;
			}

			if first <= range.last && last >= range.first {
				if first > range.first {
					let mut head = range.clone();
					head.last = first - 1;
					new_entries.push(head);
					range.first = first;
				}
				first = range.last + 1;

				if last < range.last {
					let mut tail = range.clone();
					tail.first = last + 1;
					new_entries.push(tail);
					range.last = last;
				}

				updater(&mut range.value);
			}

			if first > last {
				break;
			}
		}
		if first <= last {
			let mut value = T::default();
			updater(&mut value);
			new_entries.push(CodepointRange { first, last, value });
		}
		self.ranges.append(&mut new_entries);
		self.ranges.sort_by_key(|x| x.first);
	}

	pub fn get(&self, index: usize) -> &CodepointRange<T> {
		&self.ranges[index]
	}
}

impl<T: Default + Clone> Default for CodepointRangeMap<T> {
	fn default() -> Self {
		CodepointRangeMap {
			ranges: Default::default(),
		}
	}
}

#[cfg(test)]
mod test_codepoint_map {
	use super::*;

	#[test]
	fn default_is_empty() {
		let map: CodepointRangeMap<()> = Default::default();
		assert_eq!(map.count(), 0);
	}

	#[test]
	#[should_panic = "invalid range"]
	fn add_invalid_range_panics() {
		let mut map: CodepointRangeMap<()> = Default::default();
		map.set(20, 19, |_| {});
	}

	/// Provides a simple DSL to generate test cases for a `CodepointRangeMap`
	/// with `String` values.
	///
	/// The following verb sentences are supported:
	///
	/// - `set FIRST..LAST = VALUE`
	/// - `add FIRST..LAST = VALUE`
	/// - `check count N`
	/// - `check INDEX: FIRST..LAST = VALUE`
	///
	/// This will expand to the respective setup code and assertions.
	macro_rules! check_map {
		($($tokens:tt)*) => {
			let mut map = CodepointRangeMap::default();
			_check_map_body!(map, $($tokens)*)
		};
	}

	/// Helper macro for [`check_map`] that recursively expands the grammar.
	macro_rules! _check_map_body {
		($map:ident, ) => {};

		($map:ident, set $first:literal .. $last:literal = $value:literal $($tail:tt)*) => {
			$map.set($first, $last, |v: &mut String| *v = $value.to_string());
			_check_map_body!($map, $($tail)*)
		};

		($map:ident, add $first:literal .. $last:literal = $value:literal $($tail:tt)*) => {
			$map.set($first, $last, |v: &mut String| v.push_str($value));
			_check_map_body!($map, $($tail)*)
		};

		($map:ident, check count $count:literal $($tail:tt)*) => {
			let count = $map.count();
			if count != $count {
				panic!("expected {} range{}, it was {}", $count, if $count != 1 { "s" } else { "" }, count);
			}
			_check_map_body!($map, $($tail)*)
		};

		($map:ident, check $index:literal : $first:literal .. $last:literal = $value:literal $($tail:tt)*) => {
			let header = concat!("checking #", $index);
			if $index >= $map.count() {
				panic!("{}: index out of range", header);
			}
			let row = $map.get($index);
			if (row.first != $first || row.last != $last) {
				panic!("{}: expected range `{}..{}`, it was `{}..{}`", header, $first, $last, row.first, row.last);
			}

			if (row.value != $value) {
				panic!("{}: expected `{}..{}` = `{}`, it was `{}`", header, $first, $last, $value, row.value);
			}
			_check_map_body!($map, $($tail)*)
		};
	}

	#[test]
	fn can_insert_a_single_range() {
		check_map!(
			set 0..10 = "some range"
			check count 1
			check 0: 0..10 = "some range"
		);

		check_map!(
			set 10..20 = "other range"
			check count 1
			check 0: 10..20 = "other range"
		);
	}

	#[test]
	fn can_insert_multiple_ranges() {
		check_map!(
			set 10..20 = "a"
			set 30..40 = "b"
			check count 2
			check 0: 10..20 = "a"
			check 1: 30..40 = "b"
		);
	}

	#[test]
	fn can_modify_a_range() {
		check_map!(
			set 10..20 = "a"
			add 10..20 = "b"
			check count 1
			check 0: 10..20 = "ab"
		);

		check_map!(
			set 10..20 = "a"
			set 30..40 = "b"
			add 30..40 = "c"
			check count 2
			check 0: 10..20 = "a"
			check 1: 30..40 = "bc"
		);
	}

	#[test]
	fn set_passes_current_value_for_range() {
		let mut map = CodepointRangeMap::default();
		map.set(0, 10, |v| {
			assert_eq!(v, &0);
			*v = 100;
		});

		map.set(0, 10, |v| {
			assert_eq!(v, &100);
		});
	}

	#[test]
	fn ranges_are_sorted() {
		check_map!(
			set 40..49 = "4"
			set 10..19 = "1"
			set 90..99 = "9"
			set 60..69 = "6"
			set 70..79 = "7"
			set 80..89 = "8"
			set 30..39 = "3"
			set 20..29 = "2"
			set 50..59 = "5"
			check count 9
			check 0: 10..19 = "1"
			check 1: 20..29 = "2"
			check 2: 30..39 = "3"
			check 3: 40..49 = "4"
			check 4: 50..59 = "5"
			check 5: 60..69 = "6"
			check 6: 70..79 = "7"
			check 7: 80..89 = "8"
			check 8: 90..99 = "9"
		);
	}

	#[test]
	fn set_split_ranges_on_overlap() {
		// single overlap - start
		check_map!(
			set 10..50 = "a"
			add 10..20 = "b"
			check count 2
			check 0: 10..20 = "ab"
			check 1: 21..50 = "a"
		);

		// single overlap - end
		check_map!(
			set 10..50 = "a"
			add 20..50 = "b"
			check count 2
			check 0: 10..19 = "a"
			check 1: 20..50 = "ab"
		);

		// single overlap - middle
		check_map!(
			set 10..50 = "a"
			add 20..30 = "b"
			check count 3
			check 0: 10..19 = "a"
			check 1: 20..30 = "ab"
			check 2: 31..50 = "a"
		);

		// double overlap
		check_map!(
			set 10..29 = "a"
			set 30..50 = "b"
			add 20..40 = "c"
			check count 4
			check 0: 10..19 = "a"
			check 1: 20..29 = "ac"
			check 2: 30..40 = "bc"
			check 3: 41..50 = "b"
		);

		// triple overlap - full
		check_map!(
			set 10..19 = "a"
			set 20..29 = "b"
			set 30..39 = "c"
			add 10..39 = "d"
			check count 3
			check 0: 10..19 = "ad"
			check 1: 20..29 = "bd"
			check 2: 30..39 = "cd"
		);

		// triple overlap - contained
		check_map!(
			set 20..29 = "a"
			set 30..39 = "b"
			set 40..49 = "c"
			add 10..60 = "d"
			check count 5
			check 0: 10..19 = "d"
			check 1: 20..29 = "ad"
			check 2: 30..39 = "bd"
			check 3: 40..49 = "cd"
			check 4: 50..60 = "d"
		);

		// triple overlap - start
		check_map!(
			set 10..19 = "a"
			set 20..29 = "b"
			set 30..39 = "c"
			add 10..35 = "d"
			check count 4
			check 0: 10..19 = "ad"
			check 1: 20..29 = "bd"
			check 2: 30..35 = "cd"
			check 3: 36..39 = "c"
		);

		// triple overlap - end
		check_map!(
			set 10..19 = "a"
			set 20..29 = "b"
			set 30..39 = "c"
			add 15..39 = "d"
			check count 4
			check 0: 10..14 = "a"
			check 1: 15..19 = "ad"
			check 2: 20..29 = "bd"
			check 3: 30..39 = "cd"
		);

		// triple overlap - middle
		check_map!(
			set 10..19 = "a"
			set 20..29 = "b"
			set 30..39 = "c"
			add 15..35 = "d"
			check count 5
			check 0: 10..14 = "a"
			check 1: 15..19 = "ad"
			check 2: 20..29 = "bd"
			check 3: 30..35 = "cd"
			check 4: 36..39 = "c"
		);

		// triple overlap - spaced
		check_map!(
			set 20..30 = "a"
			set 40..50 = "b"
			set 60..70 = "c"
			add 10..80 = "d"
			check count 7
			check 0: 10..19 = "d"
			check 1: 20..30 = "ad"
			check 2: 31..39 = "d"
			check 3: 40..50 = "bd"
			check 4: 51..59 = "d"
			check 5: 60..70 = "cd"
			check 6: 71..80 = "d"
		);
	}
}
