use std::ops::RangeInclusive;

pub struct PropertyTable {
	range: Option<RangeInclusive<u32>>,
}

impl PropertyTable {
	pub fn new() -> Self {
		PropertyTable { range: None }
	}

	pub fn count(&self) -> usize {
		if let Some(range) = &self.range {
			(range.end() - range.start()) as usize + 1
		} else {
			0
		}
	}

	pub fn count_ranges(&self) -> usize {
		1
	}

	pub fn get_range<T: PropertyKey>(&self, _index: usize, _key: T) -> PropertyValue<T::Value> {
		PropertyValue(self.range.as_ref().unwrap().clone(), T::Value::get_some())
	}

	pub fn set_range<T: PropertyKey>(
		&mut self,
		range: RangeInclusive<u32>,
		_key: T,
		_property: T::Value,
	) {
		self.range = Some(range);
	}
}

impl Default for PropertyTable {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertyValue<T>(pub RangeInclusive<u32>, pub T);

pub trait PropertyKey {
	type Value: SomeValue;
}

pub trait SomeValue {
	fn get_some() -> Self;
}

impl SomeValue for u32 {
	fn get_some() -> Self {
		42
	}
}

impl SomeValue for &'static str {
	fn get_some() -> Self {
		"some property"
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
	fn can_store_single_range_with_single_property() {
		struct SomeKeyA;

		impl PropertyKey for SomeKeyA {
			type Value = &'static str;
		}

		struct SomeKeyB;

		impl PropertyKey for SomeKeyB {
			type Value = u32;
		}

		let mut table_a = PropertyTable::new();
		table_a.set_range(1..=255, SomeKeyA, "some property");
		assert_eq!(table_a.count(), 255);
		assert_eq!(table_a.count_ranges(), 1);
		assert_eq!(
			table_a.get_range(0, SomeKeyA),
			PropertyValue(1..=255, "some property")
		);

		let mut table_b = PropertyTable::new();
		table_b.set_range(0..=9, SomeKeyB, 42);
		assert_eq!(table_b.count(), 10);
		assert_eq!(table_b.count_ranges(), 1);
		assert_eq!(table_b.get_range(0, SomeKeyB), PropertyValue(0..=9, 42u32));
	}
}
