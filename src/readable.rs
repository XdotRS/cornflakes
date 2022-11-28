// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! implement {
	($($reader:ident, $ty:ty => $expr:expr),*$(,)?) => {
		$(
			impl $crate::Readable for $ty {
				fn read_from($reader: &mut impl bytes::Buf) -> Result<Self, $crate::ReadError> {
					Ok($expr)
				}
			}
		)*
	};
}

implement! {
	reader, i8 => reader.get_i8(),
	reader, i16 => reader.get_i16(),
	reader, i32 => reader.get_i32(),
	reader, i64 => reader.get_i64(),
	reader, i128 => reader.get_i128(),

	reader, u8 => reader.get_u8(),
	reader, u16 => reader.get_u16(),
	reader, u32 => reader.get_u32(),
	reader, u64 => reader.get_u64(),
	reader, u128 => reader.get_u128(),

	reader, bool => reader.get_u8() != 0,
}
