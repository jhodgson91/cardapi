pub trait HasStringCode {
    fn from_str(s: String) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn to_str(&self) -> String;
}