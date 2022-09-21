// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::reader::*;
use crate::writer::*;

pub struct Nil;

pub trait ToRwInterface {
	fn rw<I>(&self) -> I
	where
		I: RwInterface<Tail = Nil, Head = Self>,
		Self: Sized;

	fn to_rw<T: RwInterface, I>(&self, rwi: T) -> I
	where
		I: RwInterface<Head = Self, Tail = T>,
		Self: Sized;

	fn from_rwi<I>(rwi: I) -> Self
	where
		I: RwInterface<Head = Self>,
		Self: Sized,
	{
		*rwi.head()
	}
}

pub trait RwInterface {
	/// The head is the type of the last value added.
	///
	/// This is the type that created this [`RwInterface`] in particular.
	type Head;
	/// The tail is the type of all the other values added.
	type Tail;

	fn head(&self) -> &Self::Head;
	fn tail(&self) -> &Self::Tail;

	fn take(self) -> (Self::Head, Self::Tail)
	where
		Self: Sized,
	{
		(*self.head(), *self.tail())
	}

	fn new(tail: Self::Tail, head: Self::Head) -> Self;

	fn to(self, tail: Self::Tail) -> Self
	where
		Self: Sized,
	{
		Self::new(tail, *self.head())
	}
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

	fn head(&self) -> &Head {
		&self.value
	}

	fn tail(&self) -> &Tail {
		&self.tail
	}
}

pub trait DeserializeWith<Context, T = Self> {
	fn read_with(reader: &mut impl Reader, context: Context) -> T;
}

pub trait Serialize {
	fn write(self, writer: &mut impl Writer);
}

pub trait Deserialize<T = Self> {
	fn read(reader: &mut impl Reader) -> T;
}

impl<T> Serialize for T
where
	T: RwInterface,
	T::Head: Serialize,
	T::Tail: Serialize,
{
	fn write(self, writer: &mut impl Writer) {
		self.tail().write(writer);
		self.head().write(writer);
	}
}

impl<T> Deserialize for T
where
	T: RwInterface,
	T::Head: Deserialize,
	T::Tail: Deserialize,
{
	fn read(reader: &mut impl Reader) -> Self {
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
