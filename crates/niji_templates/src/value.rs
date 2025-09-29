use std::collections::HashMap;

use crate::fmt::Format;

#[derive(Debug, Default)]
pub enum Value {
	String(String),
	Bool(bool),
	Vec(Vec<Value>),
	Map(HashMap<String, Value>),
	Fmt(Box<dyn Format>),
	#[default]
	Nil
}

macro_rules! value_from_string {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Value {
                fn from(val: $ty) -> Value {
                    Value::String(val.to_string())
                }
            }
         )*
    };
}

value_from_string!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, char, String);

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Self::Bool(value)
	}
}

impl<I> FromIterator<I> for Value
where
	I: Into<Value>
{
	fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
		Self::Vec(iter.into_iter().map(Into::into).collect())
	}
}

impl<V> FromIterator<(String, V)> for Value
where
	V: Into<Value>
{
	fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
		Self::Map(iter.into_iter().map(|(k, v)| (k, v.into())).collect())
	}
}

impl<I> From<Vec<I>> for Value
where
	I: Into<Value>
{
	fn from(value: Vec<I>) -> Self {
		value.into_iter().collect()
	}
}

impl<V> From<HashMap<String, V>> for Value
where
	V: Into<Value>
{
	fn from(value: HashMap<String, V>) -> Self {
		value.into_iter().collect()
	}
}

impl<V> From<Option<V>> for Value
where
	V: Into<Value>
{
	fn from(value: Option<V>) -> Self {
		match value {
			Some(v) => v.into(),
			None => Value::Nil
		}
	}
}

impl<V> From<V> for Value
where
	V: Format + 'static
{
	fn from(value: V) -> Self {
		Self::Fmt(Box::new(value))
	}
}
