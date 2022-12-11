// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(unused)]
#![allow(incomplete_features)]
#![feature(specialization)]

use cornflakes::derive::{DataSize, StaticDataSize};

/// Here the size can be known at compile time
///
/// Regardless of the variant chosen at runtime,
/// `data_size()` should always return the size of the largest variant
#[derive(StaticDataSize)]
enum TestSizedEnum {
	Unit,
	Unnamed(u16),
	Named { field1: u32, field2: i8 },
}

/// Here the size can be known at compile time
#[derive(StaticDataSize)]
struct TestSizedStruct {
	value: u32,
	wrapper: Option<i64>,
	enum_value: TestSizedEnum,
}

#[derive(StaticDataSize)]
struct TestSizedTuple(u32, Option<i64>);

/// Here the size cannot be known at compile time,
/// So it can't implement `StaticDataSize`
#[derive(DataSize)]
enum TestDynamicEnum {
	Unit,
	Unnamed(Vec<u8>),
	Named { field1: u32, field2: Vec<i16> },
}

/// Here the size cannot be known at compile time,
/// So it can't implement `StaticDataSize`
#[derive(DataSize)]
struct TestDynamicStruct<'a> {
	value: u32,
	wrapper: Option<Vec<i32>>,
	enum_value: Vec<TestSizedEnum>,
	s: &'a [u32],
}

#[derive(DataSize)]
struct TestDynamicTuple(Vec<Option<u64>>, i64);

// We don't know if the generic implements DataSize or StaticDataSize,
// So we need both for both case.
#[derive(DataSize, StaticDataSize)]
enum TestEnumGenerics<T> {
	Unit,
	Unnamed(T),
	Named { field1: T, field2: T },
}

#[derive(DataSize, StaticDataSize)]
struct TestStructGenerics<'a, T: 'a> {
	value: &'a T,
	wrapper: Option<T>,
	enum_value: TestEnumGenerics<Option<T>>,
}

#[derive(DataSize, StaticDataSize)]
struct TestTupleGenerics<T>(Option<T>, TestEnumGenerics<T>);

// Tests

#[test]
fn test_sized_enum_unit() {
	let data = TestSizedEnum::Unit;
	assert_eq!(<TestSizedEnum as cornflakes::DataSize>::data_size(&data), 5);
}

#[test]
fn test_sized_enum_unnamed() {
	let data = TestSizedEnum::Unnamed(u16::default());
	assert_eq!(<TestSizedEnum as cornflakes::DataSize>::data_size(&data), 5);
}

#[test]
fn test_sized_enum_named() {
	let data = TestSizedEnum::Named {
		field1: u32::default(),
		field2: i8::default(),
	};
	assert_eq!(<TestSizedEnum as cornflakes::DataSize>::data_size(&data), 5);
}

#[test]
fn test_sized_struct() {
	let data = TestSizedStruct {
		value: u32::default(),
		wrapper: None,
		enum_value: TestSizedEnum::Unit,
	};
	assert_eq!(
		<TestSizedStruct as cornflakes::DataSize>::data_size(&data),
		17
	);
}

#[test]
fn test_sized_tuple() {
	let data = TestSizedTuple(u32::default(), None);
	assert_eq!(
		<TestSizedTuple as cornflakes::DataSize>::data_size(&data),
		12
	);
}

#[test]
fn test_dynamic_enum_unit() {
	let data = TestDynamicEnum::Unit;
	assert_eq!(
		<TestDynamicEnum as cornflakes::DataSize>::data_size(&data),
		1
	);
}

#[test]
fn test_dynamic_enum_unnamed() {
	let data = TestDynamicEnum::Unnamed(Vec::from([u8::default(), u8::default()]));
	assert_eq!(
		<TestDynamicEnum as cornflakes::DataSize>::data_size(&data),
		2
	);
}

#[test]
fn test_dynamic_enum_named() {
	let data = TestDynamicEnum::Named {
		field1: u32::default(),
		field2: vec![i16::default(); 10],
	};
	assert_eq!(
		<TestDynamicEnum as cornflakes::DataSize>::data_size(&data),
		24
	);
}

#[test]
fn test_dynamic_struct() {
	let data = TestDynamicStruct {
		value: u32::default(),
		wrapper: Some(Vec::from([i32::default(), i32::default()])),
		enum_value: Vec::from([TestSizedEnum::Unit]),
		s: &[u32::default()],
	};
	assert_eq!(
		<TestDynamicStruct as cornflakes::DataSize>::data_size(&data),
		21
	);
}

#[test]
fn test_dynamic_tuple() {
	let data = TestDynamicTuple(vec![None; 10], i64::default());
	assert_eq!(
		<TestDynamicTuple as cornflakes::DataSize>::data_size(&data),
		88
	);
}

#[test]
fn test_enum_with_sized_generics_unit() {
	let data = TestEnumGenerics::<u32>::Unit;
	assert_eq!(
		<TestEnumGenerics<u32> as cornflakes::DataSize>::data_size(&data),
		8
	);
}

#[test]
fn test_enum_with_sized_generics_unnamed() {
	let data = TestEnumGenerics::<u8>::Unnamed(u8::default());
	assert_eq!(
		<TestEnumGenerics<u8> as cornflakes::DataSize>::data_size(&data),
		2
	);
}

#[test]
fn test_enum_with_sized_generics_named() {
	let data = TestEnumGenerics::<i64>::Named {
		field1: i64::default(),
		field2: i64::default(),
	};
	assert_eq!(
		<TestEnumGenerics<i64> as cornflakes::DataSize>::data_size(&data),
		16
	);
}

#[test]
fn test_enum_with_dynamic_generics_unit() {
	let data = TestEnumGenerics::<Vec<u32>>::Unit;
	assert_eq!(
		<TestEnumGenerics<Vec<u32>> as cornflakes::DataSize>::data_size(&data),
		1
	);
}

#[test]
fn test_enum_with_dynamic_generics_unnamed() {
	let data = TestEnumGenerics::<Vec<u8>>::Unnamed(Vec::from([u8::default(), u8::default()]));
	assert_eq!(
		<TestEnumGenerics<Vec<u8>> as cornflakes::DataSize>::data_size(&data),
		2
	);
}

#[test]
fn test_enum_with_dynamic_generics_named() {
	let data = TestEnumGenerics::<Vec<i64>>::Named {
		field1: vec![i64::default()],
		field2: vec![i64::default(); 10],
	};
	assert_eq!(
		<TestEnumGenerics<Vec<i64>> as cornflakes::DataSize>::data_size(&data),
		88
	);
}

#[test]
fn test_struct_with_sized_generics() {
	let data = TestStructGenerics::<u16> {
		value: &u16::default(),
		wrapper: None,
		enum_value: TestEnumGenerics::Unit,
	};
	assert_eq!(
		<TestStructGenerics<u16> as cornflakes::DataSize>::data_size(&data),
		8
	);
}

#[test]
fn test_struct_with_dynamic_generics() {
	let data = TestStructGenerics::<Vec<u8>> {
		value: &vec![u8::default()],
		wrapper: Some(vec![u8::default(); 2]),
		enum_value: TestEnumGenerics::Unit,
	};
	assert_eq!(
		<TestStructGenerics<Vec<u8>> as cornflakes::DataSize>::data_size(&data),
		4
	);
}

#[test]
fn test_tuple_with_sized_generics() {
	let data = TestTupleGenerics::<i8>(None, TestEnumGenerics::Unit);
	assert_eq!(
		<TestTupleGenerics<i8> as cornflakes::DataSize>::data_size(&data),
		3
	);
}

#[test]
fn test_tuple_with_dynamic_generics() {
	let data = TestTupleGenerics::<Vec<i8>>(Some(vec![i8::default(); 10]), TestEnumGenerics::Unit);
	assert_eq!(
		<TestTupleGenerics<Vec<i8>> as cornflakes::DataSize>::data_size(&data),
		11
	);
}
