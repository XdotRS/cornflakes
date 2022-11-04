// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Deny the following clippy lints to enforce them:
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints, rather than denying them.
#![warn(clippy::use_self)]
// Warn for pedantic & cargo lints. They are allowed completely by default.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
// Continue to allow these though.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

use std::error::Error;
use bytes::{Buf, BufMut};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

pub trait DataSize {
	/// Returns the size of `self` in bytes when written with [`Writable`].
	fn data_size(&self) -> usize;
}

/// Reads a type from bytes.
pub trait Readable {
	/// Reads [`Self`] from a [`Buf`] of bytes.
	fn read_from(reader: &mut impl Buf) -> Result<Self> where Self: Sized;
}

/// Allows the reading of a type from bytes given some additional
/// [`Context`](Self::Context).
pub trait ContextualReadable {
	/// The type of context with which this type can be read from bytes.
	///
	/// For example, this might be `usize` for some collection, where that
	/// `usize` context represents the length of the list with which to read.
	type Context;

	/// Reads [`Self`] from a [`Buf`] of bytes, given some additional
	/// [`Context`](Self::Context).
	fn read_with(reader: &mut impl Buf, context: &Self::Context) -> Result<Self> where Self: Sized;
}

/// Allows a type to be written as bytes.
pub trait Writable {
	/// Writes [`self`](Self) as bytes to a [`BufMut`].
	fn write_to(&self, writer: &mut impl BufMut) -> Result where Self: Sized;
}

// This function is unused, but writing it here asserts that these traits are
// _object safe_; that is, that the Rust compiler will generate an error if any
// of these traits are accidentally made _object unsafe_, which means that they
// cannot be used with the `dyn` keyword.
fn _assert_object_safety(
	_data_size: &dyn DataSize,
	_readable: &dyn Readable,
	_contextual_readable: &dyn ContextualReadable<Context=()>,
	_writable: &dyn Writable,
) {
}
