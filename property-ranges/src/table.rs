use std::{
	any::Any,
	ops::{Range, RangeInclusive},
};

use crate::CodepointRangeMap;

pub struct PropertyTable {
	ranges: CodepointRangeMap<Properties>,
}

impl PropertyTable {
	pub fn new() -> Self {
		PropertyTable {
			ranges: Default::default(),
		}
	}

	pub fn count_ranges(&self) -> usize {
		self.ranges.count()
	}

	pub fn get_range(&self, index: usize) -> PropertyRange {
		let range = self.ranges.get(index);
		PropertyRange {
			first: range.first,
			last: range.last,
			properties: &range.value,
		}
	}

	pub fn set_range<R: CodeRange, T: PropertyKey>(
		&mut self,
		range: R,
		key: T,
		property: T::Value,
	) {
		let sta = range.start();
		let end = range.end_inclusive();
		self.ranges.set(sta, end, |values| {
			values.set(key.clone(), property.clone());
		});
	}
}

pub struct PropertyRange<'a> {
	pub first: u32,
	pub last: u32,
	properties: &'a Properties,
}

impl<'a> PropertyRange<'a> {
	pub fn get<T: PropertyKey + 'static>(&self, key: T) -> Option<T::Value> {
		self.properties.get(key)
	}
}

impl Default for PropertyTable {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Properties {
	values: Vec<(Box<dyn BaseProperty>, Box<dyn Any>)>,
}

impl Properties {
	pub(crate) fn new() -> Self {
		Properties {
			values: Default::default(),
		}
	}

	pub fn get<T: PropertyKey + 'static>(&self, key: T) -> Option<T::Value> {
		let key = key.as_base();
		for (my_key, val) in self.values.iter() {
			if my_key.is_same(&key) {
				let val = val.downcast_ref::<T::Value>();
				return Some(val.unwrap().clone());
			}
		}
		None
	}

	fn set<T: PropertyKey + 'static>(&mut self, key: T, value: T::Value) {
		let key = key.as_base();
		let value = T::value_to_any(value);
		for (my_key, val) in self.values.iter_mut() {
			if my_key.is_same(&key) {
				*val = value;
				return;
			}
		}
		self.values.push((key, value));
	}
}

impl Default for Properties {
	fn default() -> Self {
		Properties::new()
	}
}

impl Clone for Properties {
	fn clone(&self) -> Self {
		let mut clone = Properties::new();
		for (my_key, val) in self.values.iter() {
			clone
				.values
				.push((my_key.clone_self(), my_key.clone_value(val)));
		}
		clone
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertyValue<T>(pub RangeInclusive<u32>, pub T);

pub trait BaseProperty {
	fn is_same(&self, other: &Box<dyn BaseProperty>) -> bool;
	fn as_any(&self) -> Box<dyn Any>;
	fn clone_self(&self) -> Box<dyn BaseProperty>;
	fn clone_value(&self, value: &Box<dyn Any>) -> Box<dyn Any>;
}

pub trait PropertyKey: BaseProperty + PartialEq + Clone + 'static {
	type Value: Clone + PartialEq;

	fn as_base(&self) -> Box<dyn BaseProperty> {
		Box::new(self.clone())
	}

	fn value_to_any(value: Self::Value) -> Box<dyn Any> {
		Box::new(value)
	}
}

impl<T: PropertyKey> BaseProperty for T {
	fn is_same(&self, other: &Box<dyn BaseProperty>) -> bool {
		if let Some(other) = other.as_any().downcast_ref::<Self>() {
			other == self
		} else {
			false
		}
	}

	fn as_any(&self) -> Box<dyn Any> {
		Box::new(self.clone())
	}

	fn clone_self(&self) -> Box<dyn BaseProperty> {
		Box::new(self.clone())
	}

	fn clone_value(&self, value: &Box<dyn Any>) -> Box<dyn Any> {
		let value = value.downcast_ref::<T::Value>().unwrap();
		Box::new(value.clone())
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
		assert_eq!(empty.count_ranges(), 0);
	}

	#[test]
	fn supports_default() {
		let empty: PropertyTable = Default::default();
		assert_eq!(empty.count_ranges(), 0);
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
		assert_eq!(table_a.count_ranges(), 1);

		let row_a = table_a.get_range(0);
		assert_eq!(row_a.first, 1);
		assert_eq!(row_a.last, 255);
		assert_eq!(row_a.get(SomeKeyA), Some("some property"));

		let mut table_b = PropertyTable::new();
		table_b.set_range(0..=9, SomeKeyB, 42);
		assert_eq!(table_b.count_ranges(), 1);

		let row_b = table_b.get_range(0);
		assert_eq!(row_b.first, 0);
		assert_eq!(row_b.last, 9);
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
		assert_eq!(a.first, 10);
		assert_eq!(a.last, 19);
		assert_eq!(b.first, 30);
		assert_eq!(b.last, 39);
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
		assert_eq!(table.get_range(0).first, 0);
		assert_eq!(table.get_range(0).last, 9);
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

	macro_rules! check_table {
		($($tokens:tt)*) => {
			let mut table = PropertyTable::new();
			_check_table_body!(table, $($tokens)*)
		};
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
			assert_eq!(row.first..=row.last, $range,
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

	#[test]
	fn stores_entries_sorted_by_range() {
		check_table!(
			set 40..=49 => "x" = 4,
			set 10..=19 => "x" = 1,
			set 90..=99 => "x" = 9,
			set 60..=69 => "x" = 6,
			set 70..=79 => "x" = 7,
			set 80..=89 => "x" = 8,
			set 30..=39 => "x" = 3,
			set 20..=29 => "x" = 2,
			set 50..=59 => "x" = 5,
			check 9 ranges,
			check 0 as 10..=19 => { "x" = Some(1) }
			check 1 as 20..=29 => { "x" = Some(2) }
			check 2 as 30..=39 => { "x" = Some(3) }
			check 3 as 40..=49 => { "x" = Some(4) }
			check 4 as 50..=59 => { "x" = Some(5) }
			check 5 as 60..=69 => { "x" = Some(6) }
			check 6 as 70..=79 => { "x" = Some(7) }
			check 7 as 80..=89 => { "x" = Some(8) }
			check 8 as 90..=99 => { "x" = Some(9) }
		);
	}

	#[test]
	fn splits_a_range_when_setting_with_overlap() {
		check_table!(
			set 10..=90 => "x" = 1,
			set 30..=40 => "x" = 2,
			check 3 ranges,
			check 0 as 10..=29 => { "x" = Some(1) }
			check 1 as 30..=40 => { "x" = Some(2) }
			check 2 as 41..=90 => { "x" = Some(1) }
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
