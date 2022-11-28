// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! implement {
	($($ty:ty => Buf::$fun:ident()),*$(,)?) => {
		$(
			impl $crate::Readable for $ty {
				fn read_from(reader: &mut impl bytes::Buf) -> Result<Self, $crate::ReadError> {
					Ok(reader.$fun())
				}
			}
		)*
	};
}

implement! {
	i8 => Buf::get_i8(),
	i16 => Buf::get_i16(),
	i32 => Buf::get_i32(),
	i64 => Buf::get_i64(),
	i128 => Buf::get_i128(),

	u8 => Buf::get_u8(),
	u16 => Buf::get_u16(),
	u32 => Buf::get_u32(),
	u64 => Buf::get_u64(),
	u128 => Buf::get_u128(),
}
