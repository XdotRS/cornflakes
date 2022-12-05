use cornflakes::derive::Readable;

#[derive(Readable)]
enum TestEnum {
	Unit,
	Unnamed(u16),
}

#[derive(Readable)]
enum TestEnumGenerics<T> {
	Unit,
	Unnamed(T),
}

#[derive(Readable)]
enum TestNamedEnum {
	Unit,
	Named { field1: u8, field2: u8 },
}

#[derive(Readable)]
enum TestNamedEnumGenerics<T> {
	Unit,
	Named { field1: T, field2: u8 },
}

#[derive(Readable)]
struct TestStruct<T> {
	value: i32,
	value_generic: T,
	wrapper: Option<u8>,
	wrapper_generic: Option<T>,
	enum_value: TestEnum,
	enum_generic_value: TestEnumGenerics<T>,
}

#[derive(Readable)]
struct TestTuple<T>(u32, T, Option<i64>, Option<T>, TestStruct<T>);

// --=+=--
//  Tests
// --=+=--

// Simple Enums

#[test]
fn test_readable_enum_unit() {
	let data: &[u8] = &b"\x00\x00"[..];
	let r#enum = TestEnum::Unit;
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_enum_unnamed() {
	let data: &[u8] = &b"\x00\xFF"[..];
	let r#enum = TestEnum::Unnamed(0x00FF);
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_enum_gereric_unit() {
	let data: &[u8] = &b"\x00\x00\x00\x00"[..];
	let r#enum = TestEnumGenerics::Unit::<i32>;
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_enum_gereric_unnamed() {
	let data: &[u8] = &b"\x00\x02\xAB\x98"[..];
	let r#enum = TestEnumGenerics::Unnamed::<u32>(175000);
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

// Named Enums

#[test]
fn test_readable_named_enum_unit() {
	let data: &[u8] = &b"\x00\x00"[..];
	let r#enum = TestNamedEnum::Unit;
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_named_enum_named() {
	let data: &[u8] = &b"\x00\xFF"[..];
	let r#enum = TestNamedEnum::Named {
		field1: 0,
		field2: 0xFF,
	};
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_named_enum_gereric_unit() {
	let data: &[u8] = &b"\x00\x00\x00"[..];
	let r#enum = TestNamedEnumGenerics::Unit::<i16>;
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
}

#[test]
fn test_readable_named_enum_gereric_named() {
	let data: &[u8] = &b"\x00\x00\x02\xAB\x98"[..];
	let r#enum = TestNamedEnumGenerics::Named::<u32> {
		field1: 0x2AB,
		field2: 0x98,
	};
	assert_eq!(TestEnum::read_from(data).unwrap(), r#enum);
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
		enum_value: TestEnum::Unit,
		enum_generic_value: TestEnumGenerics::Unit,
	};
	assert_eq!(TestEnum::read_from(data).unwrap(), r#struct);
}

#[test]
fn test_readable_tuple() {
	let data: &[u8] = &b"\x00\x00\x00\x01\x30\x00\x20\x00\x00\x00"[..];
	let r#struct = TestStruct::<u8> {
		value: 1,
		value_generic: 48,
		wrapper: None,
		wrapper_generic: Some(32),
		enum_value: TestEnum::Unit,
		enum_generic_value: TestEnumGenerics::Unit,
	};
	assert_eq!(TestEnum::read_from(data).unwrap(), r#struct);
}
