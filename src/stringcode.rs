use rocket::http::RawStr;
use rocket::request::FromFormValue;

pub trait HasStringCode {
    fn from_str(s: String) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn to_str(&self) -> String;
}

#[derive(Debug)]
pub struct StringCodes<T: HasStringCode> {
    _inner: Vec<T>,
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

impl<'v, T: HasStringCode + Eq> FromFormValue<'v> for StringCodes<T> {
    type Error = super::common::CardAPIError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        StringCodes::from_str(form_value.to_string()).ok_or(super::common::CardAPIError::NotFound)
    }
}
