use std::mem::MaybeUninit;

use crate::well_known_types;

pub fn as_uninit(bytes: &[u8]) -> &[MaybeUninit<u8>] {
    // SAFETY: MaybeUninit<T> is guaranteed to have the same size, alignment, and ABI as T
    unsafe {
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        std::slice::from_raw_parts(ptr.cast(), len)
    }
}

pub fn as_uninit_mut(bytes: &mut [u8]) -> &mut [MaybeUninit<u8>] {
    // SAFETY: MaybeUninit<T> is guaranteed to have the same size, alignment, and ABI as T
    unsafe {
        let len = bytes.len();
        let ptr = bytes.as_mut_ptr();
        std::slice::from_raw_parts_mut(ptr.cast(), len)
    }
}

// bool <-> BoolValue

impl From<well_known_types::BoolValue> for bool {
    fn from(inner: well_known_types::BoolValue) -> Self {
        inner.value
    }
}

impl From<bool> for well_known_types::BoolValue {
    fn from(inner: bool) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// Vec<u8> <-> BytesValue

impl From<well_known_types::BytesValue> for Vec<u8> {
    fn from(inner: well_known_types::BytesValue) -> Self {
        inner.value
    }
}

impl From<Vec<u8>> for well_known_types::BytesValue {
    fn from(inner: Vec<u8>) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// f64 <-> DoubleValue

impl From<well_known_types::DoubleValue> for f64 {
    fn from(inner: well_known_types::DoubleValue) -> Self {
        inner.value
    }
}

impl From<f64> for well_known_types::DoubleValue {
    fn from(inner: f64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// f32 <-> FloatValue

impl From<well_known_types::FloatValue> for f32 {
    fn from(inner: well_known_types::FloatValue) -> Self {
        inner.value
    }
}

impl From<f32> for well_known_types::FloatValue {
    fn from(inner: f32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// i32 <-> Int32Value

impl From<well_known_types::Int32Value> for i32 {
    fn from(inner: well_known_types::Int32Value) -> Self {
        inner.value
    }
}

impl From<i32> for well_known_types::Int32Value {
    fn from(inner: i32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// i64 <-> Int64Value

impl From<well_known_types::Int64Value> for i64 {
    fn from(inner: well_known_types::Int64Value) -> Self {
        inner.value
    }
}

impl From<i64> for well_known_types::Int64Value {
    fn from(inner: i64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// u32 <-> UInt32Value

impl From<well_known_types::UInt32Value> for u32 {
    fn from(inner: well_known_types::UInt32Value) -> Self {
        inner.value
    }
}

impl From<u32> for well_known_types::UInt32Value {
    fn from(inner: u32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// u64 <-> UInt64Value

impl From<well_known_types::UInt64Value> for u64 {
    fn from(inner: well_known_types::UInt64Value) -> Self {
        inner.value
    }
}

impl From<u64> for well_known_types::UInt64Value {
    fn from(inner: u64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// () <-> Empty

impl From<well_known_types::Empty> for () {
    fn from(_inner: well_known_types::Empty) -> Self {}
}

impl From<()> for well_known_types::Empty {
    fn from(_inner: ()) -> Self {
        Self::new()
    }
}

// TODO Think about `std::time::Duration` and `std::time::SystemTime` conversions.
