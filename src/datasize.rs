// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{DataSize, StaticDataSize};

// Implementations for primitive types used in xrb

/// Simple macro for easely defining size for primitive types
macro_rules! static_type_size {
	($type:ty) => {
		impl StaticDataSize for $type {
			fn static_data_size() -> usize {
				std::mem::size_of::<$type>()
			}
		}
		impl DataSize for $type {
			fn data_size(&self) -> usize {
				Self::static_data_size()
			}
		}
	};
}

static_type_size!(bool);
static_type_size!(u8);
static_type_size!(i8);
static_type_size!(u16);
static_type_size!(i16);
static_type_size!(u32);
static_type_size!(i32);
static_type_size!(u64);
static_type_size!(i64);
static_type_size!(f32);
static_type_size!(f64);

impl<T: DataSize> DataSize for Vec<T> {
	fn data_size(&self) -> usize {
		self.iter().map(|v| v.data_size()).sum()
	}
}

impl<T: DataSize> DataSize for &[T] {
	fn data_size(&self) -> usize {
		let size: &mut usize = &mut 0;
		for e in *self {
			*size += e.data_size();
		}
		*size
	}
}

impl<T: DataSize> DataSize for [T] {
	fn data_size(&self) -> usize {
		let size: &mut usize = &mut 0;
		for e in self {
			*size += e.data_size();
		}
		*size
	}
}

impl DataSize for &str {
	fn data_size(&self) -> usize {
		self.len()
	}
}

impl<T: DataSize> DataSize for Option<T> {
	default fn data_size(&self) -> usize {
		match &self {
			None => 1,
			Some(v) => v.data_size(),
		}
	}
}

impl<T: DataSize + StaticDataSize> DataSize for Option<T> {
	fn data_size(&self) -> usize {
		Self::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for Option<T> {
	fn static_data_size() -> usize {
		T::static_data_size()
	}
}

// Size for references will be the same as the owned type.
// We don't need to implement DataSize for references because
// of auto-dereferencing at runtime.
impl<T: StaticDataSize> StaticDataSize for &T {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for &mut T {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for Box<T> {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

#[cfg(test)]
mod test {
	use super::DataSize;

	#[test]
	fn test_datasize_vec() {
		let data = vec![i16::default(); 100];
		assert_eq!(data.data_size(), 200);
	}

	#[test]
	fn test_datasize_option_static() {
		let data: Option<u64> = None;
		assert_eq!(data.data_size(), 8);
	}

	#[test]
	fn test_datasize_option_dynamic() {
		let data: Option<Vec<i64>> = Some(vec![i64::default(); 10]);
		assert_eq!(data.data_size(), 80);
	}
}
