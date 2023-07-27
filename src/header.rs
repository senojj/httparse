#![allow(dead_code)]

use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

const HEADER_FIELD_NAME_CHARACTER_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'-', 0, 0, b'0', b'1', b'2', b'3', b'4', b'5', b'6',
    b'7', b'8', b'9', 0, 0, 0, 0, 0, 0, 0, b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I',
    b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y',
    b'Z', 0, 0, 0, 0, b'_', 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k',
    b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

const HEADER_FIELD_VALUE_CHARACTER_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, b'\t', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-',
    b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=',
    b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M',
    b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[', b'\\',
    b']', b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'|',
    b'}', b'~', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
];

/// A possible error that can occur when parsing bytes or a string into a header field name.
///
/// Indicates that invalid bytes or characters were encountered while parsing.
pub struct InvalidHeaderFieldName;

/// Represents an HTTP header field name.
///
/// A value of this type will always contain characters valid for an HTTP header field name.
#[derive(Debug)]
pub struct HeaderFieldName(Vec<u8>);

impl TryFrom<&[u8]> for HeaderFieldName {
    type Error = InvalidHeaderFieldName;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl FromStr for HeaderFieldName {
    type Err = InvalidHeaderFieldName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl HeaderFieldName {
    /// Constructs a header field name from a static string.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will be converted to null bytes.
    pub fn from_static(s: &'static str) -> Self {
        Self::from_bytes_unchecked(s.as_bytes())
    }

    /// Constructs a header field name from bytes.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will result in an error.
    pub fn from_bytes(value: &[u8]) -> Result<Self, InvalidHeaderFieldName> {
        let mut o = Vec::with_capacity(value.len());
        for b in value {
            let c = HEADER_FIELD_NAME_CHARACTER_MAP[*b as usize];
            if c == 0 {
                return Err(InvalidHeaderFieldName);
            }
            o.push(c);
        }
        Ok(Self(o))
    }

    /// Constructs a header field name from bytes.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will be converted to null bytes.
    pub fn from_bytes_unchecked(value: &[u8]) -> Self {
        let mut o = Vec::with_capacity(value.len());
        for b in value {
            o.push(HEADER_FIELD_NAME_CHARACTER_MAP[*b as usize]);
        }
        Self(o)
    }

    /// Consumes the current header field name and returns a new name that has all null bytes removed.
    ///
    /// This function may alter the length of the inner value.
    pub fn clean(self) -> Self {
        let mut o = Vec::with_capacity(self.0.len());
        let mut i = VecDeque::from(self.0);
        while let Some(v) = i.pop_front() {
            if v != 0 {
                o.push(v);
            }
        }
        Self(o)
    }

    /// Consumes the current header field name and returns the inner `Vec<u8>`
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Returns a `&[u8]` representation of the header field name.
    ///
    /// Returned bytes will always be valid characters for a header field name.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns a `&str` representation of the header field name.
    ///
    /// Returned string will always be valid for a header field name.
    pub fn as_utf8_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

enum ValueType {
    Visible(Vec<u8>),
    Secret(Vec<u8>),
}

impl Debug for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Visible(b) => unsafe { std::str::from_utf8_unchecked(b) },
            Self::Secret(_) => "<REDACTED>",
        };
        write!(f, "{s}")
    }
}

impl ValueType {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Visible(b) | Self::Secret(b) => b,
        }
    }

    fn into_inner(self) -> Vec<u8> {
        match self {
            Self::Visible(b) | Self::Secret(b) => b,
        }
    }

    fn into_secret(self) -> Self {
        match self {
            Self::Visible(b) | Self::Secret(b) => Self::Secret(b),
        }
    }

    fn into_visible(self) -> Self {
        match self {
            Self::Visible(b) | Self::Secret(b) => Self::Visible(b),
        }
    }
}

/// A possible error that can occur when parsing bytes or a string into a header field value.
///
/// Indicates that invalid bytes or characters were encountered while parsing.
pub struct InvalidHeaderFieldValue;

/// Represents an HTTP header field value.
///
/// A value of this type will always contain characters valid for an HTTP header field value.
/// When a value is made secret, its contents will not be visible in debug output.
#[derive(Debug)]
pub struct HeaderFieldValue(ValueType);

impl TryFrom<&[u8]> for HeaderFieldValue {
    type Error = InvalidHeaderFieldValue;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl FromStr for HeaderFieldValue {
    type Err = InvalidHeaderFieldValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl HeaderFieldValue {
    /// Constructs a header field value from a static string.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will be converted to null bytes.
    pub fn from_static(s: &'static str) -> Self {
        Self::from_bytes_unchecked(s.as_bytes())
    }

    /// Constructs a header field value from bytes.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will result in an error.
    pub fn from_bytes(value: &[u8]) -> Result<Self, InvalidHeaderFieldValue> {
        let mut o = Vec::with_capacity(value.len());
        for b in value {
            let c = HEADER_FIELD_VALUE_CHARACTER_MAP[*b as usize];
            if c == 0 {
                return Err(InvalidHeaderFieldValue);
            }
            o.push(c);
        }
        Ok(Self(ValueType::Visible(o)))
    }

    /// Constructs a header field value from bytes.
    ///
    /// This function normalizes its input.
    /// Any invalid character bytes will be converted to null bytes.
    pub fn from_bytes_unchecked(value: &[u8]) -> Self {
        let mut o = Vec::with_capacity(value.len());
        for b in value {
            o.push(HEADER_FIELD_VALUE_CHARACTER_MAP[*b as usize]);
        }
        Self(ValueType::Visible(o))
    }

    /// Returns a `&[u8]` representation of the header field value.
    ///
    /// Returned bytes will always be valid characters for a header field value.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Consumes the current header field value and returns a new value that has all null bytes removed.
    ///
    /// This function may alter the length of the inner value.
    pub fn clean(self) -> Self {
        let is_secret = self.is_secret();
        let inner = self.0.into_inner();
        let mut o = Vec::with_capacity(inner.len());
        let mut i = VecDeque::from(inner);
        while let Some(v) = i.pop_front() {
            if v != 0 {
                o.push(v);
            }
        }
        Self(if is_secret {
            ValueType::Secret(o)
        } else {
            ValueType::Visible(o)
        })
    }

    /// Consumes the current header field value and returns the inner `Vec<u8>`
    pub fn into_inner(self) -> Vec<u8> {
        self.0.into_inner()
    }

    /// Returns a `HeaderFieldValue` that will no longer show its value in debug output.
    pub fn into_secret(self) -> Self {
        Self(self.0.into_secret())
    }

    /// Returns `true` if the value is sensitive in nature and should be handled with care.
    pub fn is_secret(&self) -> bool {
        matches!(self.0, ValueType::Secret(_))
    }

    /// Returns a `HeaderFieldValue` that will show its value in debug output.
    ///
    /// This is the default for a header field value.
    pub fn into_visible(self) -> Self {
        Self(self.0.into_visible())
    }

    /// Returns a `&str` representation of the header field value.
    ///
    /// Returned string will always be valid for a header field value.
    pub fn as_utf8_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_from_valid_chars_unchecked_as_bytes() {
        let actual = HeaderFieldName::from_static("test");
        assert_eq!("test".as_bytes(), actual.as_bytes());
    }

    #[test]
    fn name_from_invalid_chars_unchecked_as_bytes() {
        let actual = HeaderFieldName::from_static("2±1+1");
        assert_eq!("2\x00\x001\x001".as_bytes(), actual.as_bytes());
    }

    #[test]
    fn name_from_invalid_chars_unchecked_as_utf8_str() {
        let actual = HeaderFieldName::from_static("2±1+1");
        assert_eq!("2\x00\x001\x001", actual.as_utf8_str());
    }

    #[test]
    fn name_from_invalid_chars_unchecked_as_utf8_str_clean() {
        let actual = HeaderFieldName::from_static("2±1+1").clean();
        assert_eq!("211", actual.as_utf8_str());
    }

    #[test]
    fn secret_value_redacted_debug_output() {
        let actual = HeaderFieldValue::from_static("test").into_secret();
        assert_eq!("HeaderFieldValue(<REDACTED>)", &format!("{actual:?}"));
    }

    #[test]
    fn visible_value_redacted_debug_output() {
        let actual = HeaderFieldValue::from_static("test");
        assert_eq!("HeaderFieldValue(test)", &format!("{actual:?}"));
    }
}
