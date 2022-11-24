#![allow(unused)]

use cornflakes::datasize::{
	derive::{DataSize, StaticDataSize},
	DataSize, StaticDataSize,
};

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

// TODO: Do we need this ?
// #[derive(StaticDataSize)]
// struct TestSizedTuple(u32, Option<i64>);

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
struct TestDynamicStruct {
	value: u32,
	wrapper: Option<Vec<i32>>,
	enum_value: Vec<TestSizedEnum>,
}

// TODO: Do we need this ?
// #[derive(DataSize)]
// struct TestDynamicTuple(Vec<Option<u64>>, i64);

#[test]
fn test_sized_enum_unit() {
	let data = TestSizedEnum::Unit;
	assert_eq!(data.data_size(), 5);
}

#[test]
fn test_sized_enum_unnamed() {
	let data = TestSizedEnum::Unnamed(u16::default());
	assert_eq!(data.data_size(), 5);
}

#[test]
fn test_sized_enum_named() {
	let data = TestSizedEnum::Named {
		field1: u32::default(),
		field2: i8::default(),
	};
	assert_eq!(data.data_size(), 5);
}

#[test]
fn test_sized_struct() {
	let data = TestSizedStruct {
		value: u32::default(),
		wrapper: None,
		enum_value: TestSizedEnum::Unit,
	};
	assert_eq!(data.data_size(), 17);
}

// #[test]
// fn test_sized_tuple() {
// 	let data = TestSizedTuple(u32::default(), None);
// 	assert_eq!(data.data_size(), 12);
// }

#[test]
fn test_dynamic_enum_unit() {
	let data = TestDynamicEnum::Unit;
	assert_eq!(data.data_size(), 0);
}

#[test]
fn test_dynamic_enum_unnamed() {
	let data = TestDynamicEnum::Unnamed(Vec::from([u8::default(), u8::default()]));
	assert_eq!(data.data_size(), 2);
}

#[test]
fn test_dynamic_enum_named() {
	let data = TestDynamicEnum::Named {
		field1: u32::default(),
		field2: vec![i16::default(); 10],
	};
	assert_eq!(data.data_size(), 24);
}

#[test]
fn test_dynamic_struct() {
	let data = TestDynamicStruct {
		value: u32::default(),
		wrapper: Some(Vec::from([i32::default(), i32::default()])),
		enum_value: Vec::from([TestSizedEnum::Unit]),
	};
	assert_eq!(data.data_size(), 17);
}

// #[test]
// fn test_dynamic_tuple() {
// 	let data = TestDynamicTuple(vec![None; 10], i64::default());
// 	assert_eq!(data.data_size(), 88);
// }
