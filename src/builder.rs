// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::reader::*;
use crate::writer::*;

pub struct Nil;

#[doc(notable_trait)]
pub trait ToRwInterface<Tail> {
	type Output: RwInterface<Head = Self, Tail = Tail>;

	/// Creates a new [`RwInterface`] to encode `&self`.
	fn rw(&self) -> Self::Output
	where
		Self::Output: RwInterface<Head = Self, Tail = Nil>,
		Self: Sized;

	/// Wraps the given `interface` with `&self`'s [`RwInterface`].
	fn to_rw(&self, interface: Tail) -> Self::Output
	where
		Self: Sized;
}

macro_rules! prim {
	($Name:ident: $ty:tt) => {
		fn $ty(self, value: $ty) -> $Name<Self>
		where
			Self: Sized,
		{
			$Name::new(self, value)
		}
	};
}

pub trait RwInterface {
	/// The head is the type of the last value added.
	///
	/// This is the type that created this [`RwInterface`] in particular.
	type Head;
	/// The tail is the type of all the other values added.
	type Tail;

	/// Returns a reference to the last value added.
	fn head(&self) -> &Self::Head;
	/// Returns a reference to the tail (i.e., every value added other than the
	/// one added most recently).
	fn tail(&self) -> &Self::Tail;

	/// Constructs `Self` from the given `tail` and `head`.
	fn new(tail: Self::Tail, head: Self::Head) -> Self;
	/// Consumes `self` and returns a tuple containing the head and tail.
	fn pop(self) -> (Self::Head, Self::Tail);

	prim!(Bool: bool);
	prim!(Char: char);

	prim!(U8: u8);
	prim!(U16: u16);
	prim!(U32: u32);
	prim!(U64: u64);
	prim!(U128: u128);

	prim!(I8: i8);
	prim!(I16: i16);
	prim!(I32: i32);
	prim!(I64: i64);
	prim!(I128: i128);
}

pub struct Val<Head, Tail> {
	pub value: Head,
	tail: Tail,
}

impl<Head, Tail> RwInterface for Val<Head, Tail> {
	type Head = Head;
	type Tail = Tail;

	fn new(tail: Tail, head: Head) -> Self {
		Self { value: head, tail }
	}

	fn pop(self) -> (Self::Head, Self::Tail) {
		(self.value, self.tail)
	}

	fn head(&self) -> &Head {
		&self.value
	}

	fn tail(&self) -> &Tail {
		&self.tail
	}
}

macro_rules! prim {
	($Name:ident: $ty:ty) => {
		pub struct $Name<Tail = Nil> {
			pub value: $ty,
			tail: Tail,
		}

		impl<Tail> RwInterface for $Name<Tail> {
			type Head = $ty;
			type Tail = Tail;

			fn new(tail: Tail, head: $ty) -> Self {
				Self { value: head, tail }
			}

			fn pop(self) -> (Self::Head, Self::Tail) {
				(self.value, self.tail)
			}

			fn head(&self) -> &$ty {
				&self.value
			}

			fn tail(&self) -> &Tail {
				&self.tail
			}
		}

		impl<Tail> ToRwInterface<Tail> for $ty {
			type Output = $Name<Tail>;

			fn rw(&self) -> Self::Output
			where
				Self::Output: RwInterface<Head = Self, Tail = Nil>,
				Self: Sized,
			{
				$Name::new(Nil, *self)
			}

			fn to_rw(&self, interface: Tail) -> Self::Output
			where
				Self: Sized,
			{
				$Name::new(interface, *self)
			}
		}
	};
}

prim!(Bool: bool);
prim!(Char: char);

prim!(U8: u8);
prim!(U16: u16);
prim!(U32: u32);
prim!(U64: u64);
prim!(U128: u128);

prim!(I8: i8);
prim!(I16: i16);
prim!(I32: i32);
prim!(I64: i64);
prim!(I128: i128);

/// Deserializes `T` with the given additional contextual information.
///
/// For example, this is used to deserialize lists of values: the length of the
/// list is given by the `context`, allowing the correct number of elements to
/// be read.
pub trait DeserializeWith<Context, T = Self> {
	fn read_with(reader: &mut impl Reader, context: Context) -> T;
}

/// Serializes `&self` to bytes with a [`Writer`].
pub trait Serialize {
	fn write(&self, writer: &mut impl Writer);
}

/// Deserializes `T` with a ]`Reader`\.
pub trait Deserialize<T = Self> {
	fn read(reader: &mut impl Reader) -> T;
}

impl<T> Serialize for T
where
	T: RwInterface,
	T::Head: Serialize,
	T::Tail: Serialize,
{
	fn write(&self, writer: &mut impl Writer) {
		self.tail().write(writer); // write the rest of the list (the tail)
		self.head().write(writer); // write the head value
	}
}

impl<T> Deserialize for T
where
	T: RwInterface,
	T::Head: Deserialize,
	T::Tail: Deserialize,
{
	fn read(reader: &mut impl Reader) -> Self {
		// Self::new(reader.read(), reader.read())
		Self::new(T::Tail::read(reader), T::Head::read(reader))
	}
}

impl<T, Context> DeserializeWith<Context> for T
where
	T: RwInterface,
	T::Head: DeserializeWith<Context>,
	T::Tail: Deserialize,
{
	fn read_with(reader: &mut impl Reader, context: Context) -> Self {
		Self::new(T::Tail::read(reader), T::Head::read_with(reader, context))
	}
}
