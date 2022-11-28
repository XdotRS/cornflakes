// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! implement {
	($($ty:ty => $expr:expr),*$(,)?) => {
		$(
			impl $crate::Readable for $ty {
				fn read_from(reader: &mut impl bytes::Buf) -> Result<Self, $crate::ReadError> {
					Ok($expr)
				}
			}
		)*
	};
}

implement! {
	i8 => reader.get_i8(),
	i16 => reader.get_i16(),
	i32 => reader.get_i32(),
	i64 => reader.get_i64(),
	i128 => reader.get_i128(),

	u8 => reader.get_u8(),
	u16 => reader.get_u16(),
	u32 => reader.get_u32(),
	u64 => reader.get_u64(),
	u128 => reader.get_u128(),

	bool => reader.get_u8() != 0,
}
