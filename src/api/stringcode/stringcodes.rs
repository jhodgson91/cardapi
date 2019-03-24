use super::HasStringCode;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Clone, Debug)]
pub struct StringCodes<T: HasStringCode> {
    _inner: Vec<T>,
}

impl<T: HasStringCode> Default for StringCodes<T> {
    fn default() -> Self {
        StringCodes {
            _inner: Vec::new()
        }
    }
}

impl<T: HasStringCode + Eq> StringCodes<T> {
    pub fn new() -> Self {
        StringCodes { _inner: Vec::new() }
    }

    pub fn from_str(s: String) -> Option<Self> {
        let codes: Vec<&str> = s.split(",").collect();
        let mut result: Vec<T> = Vec::new();
        for code in codes {
            result.push(T::from_str(code.to_string())?);
        }
        Some(StringCodes { _inner: result })
    }

    pub fn contains(&self, other: &T) -> bool {
        self._inner.contains(other)
    }

    pub fn len(&self) -> usize {
        self._inner.len()
    }
}

impl<T: HasStringCode + Serialize> Serialize for StringCodes<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self._inner.len()))?;
        for item in &self._inner {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

struct StringCodesVisitor<T> {
    phantom: std::marker::PhantomData<T>,
}
impl<T> StringCodesVisitor<T> {
    pub fn new() -> Self {
        StringCodesVisitor {
            phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl<'de, T: HasStringCode + Deserialize<'de>> Visitor<'de> for StringCodesVisitor<T> {
    type Value = StringCodes<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a card code")
    }
    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(StringCodes { _inner: vec })
    }
}

impl<'de, T: HasStringCode + Deserialize<'de>> Deserialize<'de> for StringCodes<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StringCodesVisitor::<T>::new())
    }
}

impl<'v, T: HasStringCode + Eq> FromFormValue<'v> for StringCodes<T> {
    type Error = String;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        StringCodes::from_str(form_value.to_string())
            .ok_or("Failed to parse form value!".to_string())
    }
}
