// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! fnd {
	($Name:ident: $ty:tt) => {
		fn $ty(self, val: $ty) -> $Name<Self>
		where
			Self: Sized,
		{
			$Name::new(self, val)
		}
	};
}

pub trait Rw {
	fnd!(Bool: bool);
	fnd!(Char: char);

	fnd!(U8: u8);
	fnd!(U16: u16);
	fnd!(U32: u32);
	fnd!(U64: u64);
	fnd!(U128: u128);

	fnd!(I8: i8);
	fnd!(I16: i16);
	fnd!(I32: i32);
	fnd!(I64: i64);
	fnd!(I128: i128);
}

macro_rules! foundational {
	($Name:ident: $ty:ty) => {
		pub struct $Name<RW>
		where
			RW: Rw,
		{
			pub(crate) rw: RW,
			pub value: $ty,
		}

		impl<RW> $Name<RW>
		where
			RW: Rw,
		{
			pub fn new(rw: RW, value: $ty) -> Self {
				Self { rw, value }
			}
		}
	};
}

foundational!(Bool: bool);
foundational!(Char: char);

foundational!(U8: u8);
foundational!(U16: u16);
foundational!(U32: u32);
foundational!(U64: u64);
foundational!(U128: u128);

foundational!(I8: i8);
foundational!(I16: i16);
foundational!(I32: i32);
foundational!(I64: i64);
foundational!(I128: i128);
