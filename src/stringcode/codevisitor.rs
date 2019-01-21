use super::HasStringCode;

use serde::de::Visitor;

pub struct CodeVisitor<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> CodeVisitor<T> {
    pub fn new() -> Self {
        CodeVisitor {
            phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl<'de, T: HasStringCode> Visitor<'de> for CodeVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a card code")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match T::from_str(value.to_string()) {
            Some(s) => Ok(s),
            None => Err(E::custom("Invalid card code")),
        }
    }
}
