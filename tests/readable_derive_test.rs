// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(incomplete_features)]
#![feature(specialization)]

use cornflakes::derive::{DataSize, Readable, StaticDataSize};

#[derive(Debug, PartialEq, Eq, Readable, StaticDataSize)]
enum TestEnumWrapper {
	Unit,
	Unnamed(u16),
}

// for reference
//
// impl cornflakes::Readable for TestEnumWrapper {
// 	fn read_from(reader: &mut impl bytes::Buf) -> cornflakes::ReadResult<Self> {
// 		let __0__ = <u16 as cornflakes::Readable>::read_from(reader)?;
// 		Ok(match __0__ {
// 			0 => Self::Unit,
// 			v => Self::Unnamed(v),
// 		})
// 	}
// }

// Is it really possible to do it here ? or do we do it manually ?
#[derive(Debug, PartialEq, Eq, Readable, DataSize, StaticDataSize)]
enum TestEnumWrapperGenerics<T> {
	Unit,
	Unnamed(T),
}

// for reference
//
// impl<T: cornflakes::Readable + cornflakes::StaticDataSize> cornflakes::Readable for TestEnumWrapperGenerics<T> {
// 	fn read_from(reader: &mut impl bytes::Buf) -> cornflakes::ReadResult<Self> {
// 		let datasize = <T as cornflakes::StaticDataSize>::static_data_size();
// 		let raw: &[u8] = reader.take(datasize).get_u8;
// 		let __0__ = <T as cornflakes::Readable>::read_from(reader)?;
// 		Ok(match __0__ {
// 			0 => Self::Unit,
// 			v => Self::Unnamed(v),
// 		})
// 	}
// }

#[derive(Debug, PartialEq, Eq, StaticDataSize, Readable)]
#[repr(u8)]
enum TestEnumUnits {
	Variant0,
	Variant1,
	Variant2,
	Variant3,
	Variant4,
	Variant5,
}

// for reference
//
// impl cornflakes::Readable for TestEnumUnits {
// 	fn read_from(reader: &mut impl bytes::Buf) -> cornflakes::ReadResult<Self> {
// 		let __0__ = <u8 as cornflakes::Readable>::read_from(reader)?;
// 		Ok(match __0__ {
// 			0 => Self::Variant0,
// 			1 => Self::Variant1,
// 			2 => Self::Variant2,
// 			4 => Self::Variant3,
// 			5 => Self::Variant4,
// 			6 => Self::Variant5,
// 		})
// 	}
// }

#[derive(Debug, PartialEq, Eq, DataSize, StaticDataSize, Readable)]
struct TestStruct<T> {
	value: i32,
	value_generic: T,
	wrapper: Option<u8>,
	wrapper_generic: Option<T>,
	enum_value: TestEnumWrapper,
	enum_generic_value: TestEnumWrapperGenerics<T>,
}

// for reference
//
// impl<T: cornflakes::Readable> cornflakes::Readable for TestStruct<T> {
// 	fn read_from(reader: &mut impl bytes::Buf) -> cornflakes::ReadResult<Self> {
// 		let __value__ = <i32 as cornflakes::Readable>::read_from(reader)?;
// 		let __value_generic__ = <T as cornflakes::Readable>::read_from(reader)?;
// 		let __wrapper__ = <Option<u8> as cornflakes::Readable>::read_from(reader)?;
// 		let __wrapper_generic__ = <Option<T> as cornflakes::Readable>::read_from(reader)?;
// 		let __enum_value__ = <TestEnumWrapper as cornflakes::Readable>::read_from(reader)?;
// 		let __enum_generic_value__ =
// 			<TestEnumWrapperGenerics<T> as cornflakes::Readable>::read_from(reader)?;
// 		Ok(Self {
// 			value: __value__,
// 			value_generic: __value_generic__,
// 			wrapper: __wrapper__,
// 			wrapper_generic: __wrapper_generic__,
// 			enum_value: __enum_value__,
// 			enum_generic_value: __enum_generic_value__,
// 		})
// 	}
// }

#[derive(Debug, PartialEq, Eq, DataSize, StaticDataSize, Readable)]
struct TestTuple<T>(u32, T, Option<i64>, Option<T>, TestStruct<T>);

// for reference
//
// impl<T: cornflakes::Readable> cornflakes::Readable for TestTuple<T> {
// 	fn read_from(reader: &mut impl bytes::Buf) -> cornflakes::ReadResult<Self> {
// 		let __0__ = <u32 as cornflakes::Readable>::read_from(reader)?;
// 		let __1__ = <T as cornflakes::Readable>::read_from(reader)?;
// 		let __2__ = <Option<i64> as cornflakes::Readable>::read_from(reader)?;
// 		let __3__ = <Option<T> as cornflakes::Readable>::read_from(reader)?;
// 		let __4__ = <TestStruct<T> as cornflakes::Readable>::read_from(reader)?;
// 		Ok(Self(__0__, __1__, __2__, __3__, __4__))
// 	}
// }

// --=+=--
//  Tests
// --=+=--

// Wrapper Enums

#[test]
fn test_readable_wrapper_enum_unit() {
	let data: &[u8] = &b"\x00\x00"[..];
	let r#enum = TestEnumWrapper::Unit;
	assert_eq!(
		<TestEnumWrapper as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#enum
	);
}

#[test]
fn test_readable_wrapper_enum_unnamed() {
	let data: &[u8] = &b"\x00\xFF"[..];
	let r#enum = TestEnumWrapper::Unnamed(0x00FF);
	assert_eq!(
		<TestEnumWrapper as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#enum
	);
}

#[test]
fn test_readable_wrapper_enum_gereric_unit() {
	let data: &[u8] = &b"\x00\x00\x00\x00"[..];
	let r#enum = TestEnumWrapperGenerics::Unit::<i32>;
	assert_eq!(
		<TestEnumWrapperGenerics<i32> as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#enum
	);
}

#[test]
fn test_readable_wrapper_enum_gereric_unnamed() {
	let data: &[u8] = &b"\x00\x02\xAB\x98"[..];
	let r#enum = TestEnumWrapperGenerics::Unnamed::<u32>(175000);
	assert_eq!(
		<TestEnumWrapperGenerics<u32> as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#enum
	);
}

// Unit Enums

#[test]
fn test_readable_unit_enum() {
	let data: &[u8] = &b"\x04"[..];
	let r#enum = TestEnumUnits::Variant4;
	assert_eq!(
		<TestEnumUnits as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#enum
	);
}

// Structs

#[test]
fn test_readable_struct() {
	let data: &[u8] = &b"\x00\x00\x00\x01\x30\x00\x20\x00\x00\x00"[..];
	let r#struct = TestStruct::<u8> {
		value: 1,
		value_generic: 48,
		wrapper: None,
		wrapper_generic: Some(32),
		enum_value: TestEnumWrapper::Unit,
		enum_generic_value: TestEnumWrapperGenerics::Unit,
	};
	assert_eq!(
		<TestStruct<u8> as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#struct
	);
}

#[test]
fn test_readable_tuple() {
	let data: &[u8] = &b"\x00\x00\x00\x01\x30\x00\x20\x00\x00\x00"[..];
	let r#struct = TestStruct::<u8> {
		value: 1,
		value_generic: 48,
		wrapper: None,
		wrapper_generic: Some(32),
		enum_value: TestEnumWrapper::Unit,
		enum_generic_value: TestEnumWrapperGenerics::Unit,
	};
	assert_eq!(
		<TestStruct<u8> as cornflakes::Readable>::read_from(&mut data).unwrap(),
		r#struct
	);
}
