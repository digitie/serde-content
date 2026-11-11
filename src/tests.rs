#![cfg(test)]

use crate::Error;
use crate::Value;
use core::mem::size_of;

// Ensure the value works well with derive macros.
#[cfg(feature = "derive")]
#[derive(serde::Serialize, serde::Deserialize)]
struct Bar(Value<'static>);

// Ensure the value works well with derive macros.
#[cfg(feature = "derive")]
#[derive(serde::Serialize, serde::Deserialize)]
struct Foo(Bar);

#[test]
fn value_size() {
    assert!(size_of::<Value>() <= 32);
}

#[test]
fn error_size() {
    assert!(size_of::<Error>() <= 8);
}

// 파생 타입이 실제 값의 직렬화·역직렬화에서도 중첩 Value를 보존하는지 검증한다.
#[cfg(feature = "derive")]
#[test]
fn derived_nested_value_roundtrip() {
    let original = Foo(Bar(Value::Unit));
    let encoded = crate::Serializer::new().serialize(&original).unwrap();
    let restored: Foo = crate::Deserializer::new(encoded).deserialize().unwrap();
    let Foo(Bar(original_value)) = original;
    let Foo(Bar(restored_value)) = restored;
    assert_eq!(original_value, restored_value);
}
