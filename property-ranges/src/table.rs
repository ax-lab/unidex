use std::{
	any::Any,
	ops::{Range, RangeInclusive},
};

pub struct PropertyTable {
	ranges: Vec<PropertyRange>,
}

impl PropertyTable {
	pub fn new() -> Self {
		PropertyTable {
			ranges: Default::default(),
		}
	}

	pub fn count(&self) -> usize {
		if let Some(row) = self.ranges.iter().next() {
			let range = &row.range;
			(range.end() - range.start()) as usize + 1
		} else {
			0
		}
	}

	pub fn count_ranges(&self) -> usize {
		self.ranges.len()
	}

	pub fn get_range(&self, index: usize) -> &PropertyRange {
		&self.ranges[index]
	}

	pub fn set_range<R: CodeRange, T: PropertyKey>(
		&mut self,
		range: R,
		key: T,
		property: T::Value,
	) {
		let range = RangeInclusive::new(range.start(), range.end_inclusive());
		for it in self.ranges.iter_mut() {
			if it.range == range {
				it.values.push((key.as_any(), T::value_to_any(property)));
				return;
			} else if *it.range.end() + 1 == *range.start() {
				if it.get(key.clone()) == Some(property.clone()) && it.values.len() == 1 {
					it.range = RangeInclusive::new(*it.range.start(), *range.end());
					return;
				}
			}
		}
		let mut range = PropertyRange::new(range);
		range.values.push((key.as_any(), T::value_to_any(property)));
		self.ranges.push(range);
	}
}

impl Default for PropertyTable {
	fn default() -> Self {
		Self::new()
	}
}

pub struct PropertyRange {
	pub range: RangeInclusive<u32>,
	values: Vec<(Box<dyn Any>, Box<dyn Any>)>,
}

impl PropertyRange {
	pub(crate) fn new(range: RangeInclusive<u32>) -> Self {
		PropertyRange {
			range,
			values: Default::default(),
		}
	}

	pub fn get<T: PropertyKey + 'static>(&self, key: T) -> Option<T::Value> {
		for (my_key, val) in self.values.iter() {
			if let Some(my_key) = my_key.downcast_ref::<T>() {
				if &key == my_key {
					let val = val.downcast_ref::<T::Value>();
					return Some(val.unwrap().clone());
				}
			}
		}
		None
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertyValue<T>(pub RangeInclusive<u32>, pub T);

pub trait PropertyKey: PartialEq + Clone + 'static {
	type Value: Clone + PartialEq;

	fn as_any(&self) -> Box<dyn Any> {
		Box::new(self.clone())
	}

	fn value_to_any(value: Self::Value) -> Box<dyn Any> {
		Box::new(value)
	}
}

pub trait CodeRange {
	fn start(&self) -> u32;
	fn end_inclusive(&self) -> u32;
}

impl CodeRange for Range<u32> {
	fn start(&self) -> u32 {
		self.start
	}

	fn end_inclusive(&self) -> u32 {
		self.end - 1
	}
}

impl CodeRange for RangeInclusive<u32> {
	fn start(&self) -> u32 {
		*self.start()
	}

	fn end_inclusive(&self) -> u32 {
		*self.end()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_create_empty() {
		let empty = PropertyTable::new();
		assert_eq!(empty.count(), 0);
	}

	#[test]
	fn supports_default() {
		let empty: PropertyTable = Default::default();
		assert_eq!(empty.count(), 0);
	}

	#[test]
	fn stores_single_range_with_single_property() {
		#[derive(Clone, PartialEq)]
		struct SomeKeyA;

		impl PropertyKey for SomeKeyA {
			type Value = &'static str;
		}

		#[derive(Clone, PartialEq)]
		struct SomeKeyB;

		impl PropertyKey for SomeKeyB {
			type Value = u32;
		}

		let mut table_a = PropertyTable::new();
		table_a.set_range(1..=255, SomeKeyA, "some property");
		assert_eq!(table_a.count(), 255);
		assert_eq!(table_a.count_ranges(), 1);

		let row_a = table_a.get_range(0);
		assert_eq!(row_a.range, 1..=255);
		assert_eq!(row_a.get(SomeKeyA), Some("some property"));

		let mut table_b = PropertyTable::new();
		table_b.set_range(0..=9, SomeKeyB, 42);
		assert_eq!(table_b.count(), 10);
		assert_eq!(table_b.count_ranges(), 1);

		let row_b = table_b.get_range(0);
		assert_eq!(row_b.range, 0..=9);
		assert_eq!(row_b.get(SomeKeyB), Some(42));
	}

	#[test]
	fn stores_multiple_properties_per_range() {
		#[derive(Clone, PartialEq)]
		struct Key(&'static str);

		impl PropertyKey for Key {
			type Value = &'static str;
		}

		let mut table = PropertyTable::new();
		table.set_range(0..=9, Key("a"), "value a");
		table.set_range(0..=9, Key("b"), "value b");

		let row = table.get_range(0);
		assert_eq!(row.get(Key("a")), Some("value a"));
		assert_eq!(row.get(Key("b")), Some("value b"));
	}

	#[test]
	fn stores_multiple_ranges() {
		#[derive(Clone, PartialEq)]
		struct Key;

		impl PropertyKey for Key {
			type Value = u32;
		}

		let mut table = PropertyTable::new();
		table.set_range(10..=19, Key, 1);
		table.set_range(30..=39, Key, 3);
		assert_eq!(table.count_ranges(), 2);

		let a = table.get_range(0);
		let b = table.get_range(1);
		assert_eq!(a.range, 10..=19);
		assert_eq!(b.range, 30..=39);
		assert_eq!(a.get(Key), Some(1));
		assert_eq!(b.get(Key), Some(3));
	}

	#[test]
	fn supports_non_inclusive_range() {
		#[derive(Clone, PartialEq)]
		struct Key;

		impl PropertyKey for Key {
			type Value = u32;
		}

		let mut table = PropertyTable::new();
		table.set_range(0..10, Key, 42);
		assert_eq!(table.get_range(0).range, 0..=9);
	}

	#[test]
	fn returns_none_for_unset_property() {
		impl PropertyKey for &'static str {
			type Value = u32;
		}

		let mut table = PropertyTable::new();
		table.set_range(0..10, "key", 0);

		assert_eq!(table.get_range(0).get("other key"), None);
	}

	#[derive(Clone, PartialEq)]
	struct Key(&'static str);

	impl PropertyKey for Key {
		type Value = u32;
	}

	macro_rules! _check_table_body {
		($tb:ident, ) => {};

		($tb:ident, set $range:expr => $key:literal = $val:expr, $($tail:tt)*) => {
			$tb.set_range($range, Key($key), $val);
			_check_table_body!($tb, $($tail)*)
		};

		($tb:ident, check $count:literal ranges, $($tail:tt)*) => {
			let range_count = $tb.count_ranges();
			assert_eq!(range_count, $count,
				concat!("expected ", $count, " range{}, was {}"),
				if $count != 1 { "s" } else { "" }, range_count);
			_check_table_body!($tb, $($tail)*)
		};

		($tb:ident, check $count:literal range, $($tail:tt)*) => {
			_check_table_body!($tb, check $count ranges, $($tail)*)
		};

		($tb:ident, check $index:literal as $range:expr => {
			$($key:literal = $val:expr),*
		} $($tail:tt)*) => {
			let header = concat!("checking range at ", $index);
			if $index >= $tb.count_ranges() {
				panic!("{}: no such range", header);
			}
			let row = $tb.get_range($index);
			assert_eq!(row.range, $range,
				"{}: expected range `{:?}`", header, $range);
			$(
				let actual_val = row.get(Key($key));
				let expected = $val;
				assert_eq!(actual_val, expected,
					"{}: expected `{}` = `{:?}`, was `{:?}`",
					header, $key, expected, actual_val);
			)*
			_check_table_body!($tb, $($tail)*)
		};
	}

	macro_rules! check_table {
		($($tokens:tt)*) => {
			let mut table = PropertyTable::new();
			_check_table_body!(table, $($tokens)*)
		};
	}

	#[test]
	fn merges_consecutive_ranges_with_same_properties() {
		check_table!(
			set 0..10  => "a" = 10,
			set 10..20 => "a" = 10,
			check 1 range,
			check 0 as 0..=19 => { "a" = Some(10) }
		);
	}

	#[test]
	fn does_not_merge_consecutive_ranges_with_different_values() {
		check_table!(
			set 0..10  => "a" = 10,
			set 10..20 => "a" = 20,
			check 2 ranges,
			check 0 as 0..=9   => { "a" = Some(10) }
			check 1 as 10..=19 => { "a" = Some(20) }
		);

		check_table!(
			set 0..10  => "a" = 10,
			set 0..10  => "b" = 20,
			set 10..20 => "a" = 10,
			check 2 ranges,
			check 0 as 0..=9   => { "a" = Some(10), "b" = Some(20) }
			check 1 as 10..=19 => { "a" = Some(10), "b" = None }
		);
	}
}
