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

	pub fn get<T: PropertyKey + 'static>(&self, key: T) -> T::Value {
		for (my_key, val) in self.values.iter() {
			if let Some(my_key) = my_key.downcast_ref::<T>() {
				if &key == my_key {
					let val = val.downcast_ref::<T::Value>();
					return val.unwrap().clone();
				}
			}
		}
		panic!()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertyValue<T>(pub RangeInclusive<u32>, pub T);

pub trait PropertyKey: PartialEq + Clone + 'static {
	type Value: Clone;

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

		let range_a = table_a.get_range(0);
		assert_eq!(range_a.range, 1..=255);
		assert_eq!(range_a.get(SomeKeyA), "some property");

		let mut table_b = PropertyTable::new();
		table_b.set_range(0..=9, SomeKeyB, 42);
		assert_eq!(table_b.count(), 10);
		assert_eq!(table_b.count_ranges(), 1);

		let range_b = table_b.get_range(0);
		assert_eq!(range_b.range, 0..=9);
		assert_eq!(range_b.get(SomeKeyB), 42u32);
	}

	#[test]
	fn stores_multiple_properties() {
		#[derive(Clone, PartialEq)]
		struct Key(&'static str);

		impl PropertyKey for Key {
			type Value = &'static str;
		}

		let mut table = PropertyTable::new();
		table.set_range(0..=9, Key("a"), "value a");
		table.set_range(0..=9, Key("b"), "value b");

		let range = table.get_range(0);
		assert_eq!(range.get(Key("a")), "value a");
		assert_eq!(range.get(Key("b")), "value b");
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
		table.set_range(20..=29, Key, 2);
		assert_eq!(table.count_ranges(), 2);

		let a = table.get_range(0);
		let b = table.get_range(1);
		assert_eq!(a.range, 10..=19);
		assert_eq!(b.range, 20..=29);
		assert_eq!(a.get(Key), 1);
		assert_eq!(b.get(Key), 2);
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
}
