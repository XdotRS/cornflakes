// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait DataSize {
	/// Returns the size of `self` in bytes when written with [`Writable`].
	fn data_size(&self) -> usize;
}

pub trait StaticDataSize {
	/// Returns the size of `Self` in bytes when written with [`Writable`].
	///
	/// If `Self` is an `enum`, then the size is the maximum size of the values
	/// contained in the variants
	fn static_data_size() -> usize where Self: Sized;
}

// Implementations for primitive types used in xrb

/// Simple macro for easely defining size for primitive types
macro_rules! static_type_size {
	($type:ty, $size:literal) => {
		impl StaticDataSize for $type {
			fn static_data_size() -> usize {
				$size
			}
		}
		impl DataSize for $type {
			fn data_size(&self) -> usize {
				Self::static_data_size()
			}
		}
	};
}

static_type_size!(u8, 1);
static_type_size!(i8, 1);
static_type_size!(u16, 2);
static_type_size!(i16, 2);
static_type_size!(u32, 3);
static_type_size!(i32, 3);
static_type_size!(u64, 4);
static_type_size!(i64, 4);

impl<T> DataSize for Vec<T>
where
	T: DataSize,
{
	fn data_size(&self) -> usize {
		self.iter()
			.map(|v| v.data_size())
			.fold(0, |acc, v| acc + v)
	}
}

impl<T: DataSize> DataSize for Option<T>
{
	default fn data_size(&self) -> usize {
		match &self {
			Option::None => 0,
			Option::Some(v) => v.data_size(),
		}
	}
}
impl<T: DataSize + StaticDataSize> DataSize for Option<T>
{
	fn data_size(&self) -> usize {
		Self::static_data_size()
	}
}
impl<T> StaticDataSize for Option<T>
where
	T: StaticDataSize,
{
	fn static_data_size() -> usize {
	    T::static_data_size()
	}
}
