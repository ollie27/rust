// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations on ASCII strings and characters

#![stable(feature = "rust1", since = "1.0.0")]

use prelude::v1::*;

use mem;
use ops::Range;

/// Extension methods for ASCII-subset only operations on string slices.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsciiExt {
    /// Container type for copied ASCII characters.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Owned;

    /// Checks if within the ASCII range.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ascii::AsciiExt;
    ///
    /// let ascii = 'a';
    /// let utf8 = '❤';
    ///
    /// assert_eq!(true, ascii.is_ascii());
    /// assert_eq!(false, utf8.is_ascii())
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn is_ascii(&self) -> bool;

    /// Makes a copy of the string in ASCII upper case.
    ///
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z',
    /// but non-ASCII letters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ascii::AsciiExt;
    ///
    /// let ascii = 'a';
    /// let utf8 = '❤';
    ///
    /// assert_eq!('A', ascii.to_ascii_uppercase());
    /// assert_eq!('❤', utf8.to_ascii_uppercase());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn to_ascii_uppercase(&self) -> Self::Owned;

    /// Makes a copy of the string in ASCII lower case.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z',
    /// but non-ASCII letters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ascii::AsciiExt;
    ///
    /// let ascii = 'A';
    /// let utf8 = '❤';
    ///
    /// assert_eq!('a', ascii.to_ascii_lowercase());
    /// assert_eq!('❤', utf8.to_ascii_lowercase());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn to_ascii_lowercase(&self) -> Self::Owned;

    /// Checks that two strings are an ASCII case-insensitive match.
    ///
    /// Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`,
    /// but without allocating and copying temporary strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ascii::AsciiExt;
    ///
    /// let ascii1 = 'A';
    /// let ascii2 = 'a';
    /// let ascii3 = 'A';
    /// let ascii4 = 'z';
    ///
    /// assert_eq!(true, ascii1.eq_ignore_ascii_case(&ascii2));
    /// assert_eq!(true, ascii1.eq_ignore_ascii_case(&ascii3));
    /// assert_eq!(false, ascii1.eq_ignore_ascii_case(&ascii4));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn eq_ignore_ascii_case(&self, other: &Self) -> bool;

    /// Converts this type to its ASCII upper case equivalent in-place.
    ///
    /// See `to_ascii_uppercase` for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(ascii)]
    ///
    /// use std::ascii::AsciiExt;
    ///
    /// let mut ascii = 'a';
    ///
    /// ascii.make_ascii_uppercase();
    ///
    /// assert_eq!('A', ascii);
    /// ```
    #[unstable(feature = "ascii", issue = "27809")]
    fn make_ascii_uppercase(&mut self);

    /// Converts this type to its ASCII lower case equivalent in-place.
    ///
    /// See `to_ascii_lowercase` for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(ascii)]
    ///
    /// use std::ascii::AsciiExt;
    ///
    /// let mut ascii = 'A';
    ///
    /// ascii.make_ascii_lowercase();
    ///
    /// assert_eq!('a', ascii);
    /// ```
    #[unstable(feature = "ascii", issue = "27809")]
    fn make_ascii_lowercase(&mut self);
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsciiExt for str {
    type Owned = String;

    #[inline]
    fn is_ascii(&self) -> bool {
        self.bytes().all(|b| b.is_ascii())
    }

    #[inline]
    fn to_ascii_uppercase(&self) -> String {
        let bytes = self.as_bytes().to_ascii_uppercase();
        // to_ascii_uppercase() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn to_ascii_lowercase(&self) -> String {
        let bytes = self.as_bytes().to_ascii_lowercase();
        // to_ascii_uppercase() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        self.as_bytes().eq_ignore_ascii_case(other.as_bytes())
    }

    fn make_ascii_uppercase(&mut self) {
        let me: &mut [u8] = unsafe { mem::transmute(self) };
        me.make_ascii_uppercase()
    }

    fn make_ascii_lowercase(&mut self) {
        let me: &mut [u8] = unsafe { mem::transmute(self) };
        me.make_ascii_lowercase()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsciiExt for [u8] {
    type Owned = Vec<u8>;
    #[inline]
    fn is_ascii(&self) -> bool {
        self.iter().all(|b| b.is_ascii())
    }

    #[inline]
    fn to_ascii_uppercase(&self) -> Vec<u8> {
        unsafe {
            let len = self.len();
            let mut v = Vec::with_capacity(len);
            v.set_len(len);
            for i in 0..len {
                v[i] = self[i].to_ascii_uppercase();
            }
            v
        }
    }

    #[inline]
    fn to_ascii_lowercase(&self) -> Vec<u8> {
        unsafe {
            let len = self.len();
            let mut v = Vec::with_capacity(len);
            v.set_len(len);
            for i in 0..len {
                v[i] = self[i].to_ascii_lowercase();
            }
            v
        }
    }

    #[inline]
    fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        self.len() == other.len() &&
        self.iter().zip(other).all(|(a, b)| {
            a.eq_ignore_ascii_case(b)
        })
    }

    fn make_ascii_uppercase(&mut self) {
        for byte in self {
            byte.make_ascii_uppercase();
        }
    }

    fn make_ascii_lowercase(&mut self) {
        for byte in self {
            byte.make_ascii_lowercase();
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsciiExt for u8 {
    type Owned = u8;
    #[inline]
    fn is_ascii(&self) -> bool { *self & 128 == 0 }
    #[inline]
    fn to_ascii_uppercase(&self) -> u8 {
        self - match *self {
            b'a' ... b'z' => b'a' - b'A',
            _ => 0,
        }
    }
    #[inline]
    fn to_ascii_lowercase(&self) -> u8 {
        self + match *self {
            b'A' ... b'Z' => b'a' - b'A',
            _ => 0,
        }
    }
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &u8) -> bool {
        self.to_ascii_lowercase() == other.to_ascii_lowercase()
    }
    #[inline]
    fn make_ascii_uppercase(&mut self) { *self = self.to_ascii_uppercase(); }
    #[inline]
    fn make_ascii_lowercase(&mut self) { *self = self.to_ascii_lowercase(); }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsciiExt for char {
    type Owned = char;
    #[inline]
    fn is_ascii(&self) -> bool {
        *self as u32 <= 0x7F
    }

    #[inline]
    fn to_ascii_uppercase(&self) -> char {
        if self.is_ascii() {
            (*self as u8).to_ascii_uppercase() as char
        } else {
            *self
        }
    }

    #[inline]
    fn to_ascii_lowercase(&self) -> char {
        if self.is_ascii() {
            (*self as u8).to_ascii_lowercase() as char
        } else {
            *self
        }
    }

    #[inline]
    fn eq_ignore_ascii_case(&self, other: &char) -> bool {
        self.to_ascii_lowercase() == other.to_ascii_lowercase()
    }

    #[inline]
    fn make_ascii_uppercase(&mut self) { *self = self.to_ascii_uppercase(); }
    #[inline]
    fn make_ascii_lowercase(&mut self) { *self = self.to_ascii_lowercase(); }
}

/// An iterator over the escaped version of a byte, constructed via
/// `std::ascii::escape_default`.
#[stable(feature = "rust1", since = "1.0.0")]
pub struct EscapeDefault {
    range: Range<usize>,
    data: [u8; 4],
}

/// Returns an iterator that produces an escaped version of a `u8`.
///
/// The default is chosen with a bias toward producing literals that are
/// legal in a variety of languages, including C++11 and similar C-family
/// languages. The exact rules are:
///
/// - Tab, CR and LF are escaped as '\t', '\r' and '\n' respectively.
/// - Single-quote, double-quote and backslash chars are backslash-escaped.
/// - Any other chars in the range [0x20,0x7e] are not escaped.
/// - Any other chars are given hex escapes of the form '\xNN'.
/// - Unicode escapes are never generated by this function.
///
/// # Examples
///
/// ```
/// use std::ascii;
///
/// let escaped = ascii::escape_default(b'0').next().unwrap();
/// assert_eq!(b'0', escaped);
///
/// let mut escaped = ascii::escape_default(b'\t');
///
/// assert_eq!(b'\\', escaped.next().unwrap());
/// assert_eq!(b't', escaped.next().unwrap());
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub fn escape_default(c: u8) -> EscapeDefault {
    let (data, len) = match c {
        b'\t' => ([b'\\', b't', 0, 0], 2),
        b'\r' => ([b'\\', b'r', 0, 0], 2),
        b'\n' => ([b'\\', b'n', 0, 0], 2),
        b'\\' => ([b'\\', b'\\', 0, 0], 2),
        b'\'' => ([b'\\', b'\'', 0, 0], 2),
        b'"' => ([b'\\', b'"', 0, 0], 2),
        b'\x20' ... b'\x7e' => ([c, 0, 0, 0], 1),
        _ => ([b'\\', b'x', hexify(c >> 4), hexify(c & 0xf)], 4),
    };

    return EscapeDefault { range: (0.. len), data: data };

    fn hexify(b: u8) -> u8 {
        match b {
            0 ... 9 => b'0' + b,
            _ => b'a' + b - 10,
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for EscapeDefault {
    type Item = u8;
    fn next(&mut self) -> Option<u8> { self.range.next().map(|i| self.data[i]) }
    fn size_hint(&self) -> (usize, Option<usize>) { self.range.size_hint() }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl DoubleEndedIterator for EscapeDefault {
    fn next_back(&mut self) -> Option<u8> {
        self.range.next_back().map(|i| self.data[i])
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl ExactSizeIterator for EscapeDefault {}

#[cfg(test)]
mod tests {
    use prelude::v1::*;
    use super::*;
    use char::from_u32;

    #[test]
    fn test_is_ascii() {
        assert!(b"".is_ascii());
        assert!(b"banana\0\x7F".is_ascii());
        assert!(b"banana\0\x7F".iter().all(|b| b.is_ascii()));
        assert!(!b"Vi\xe1\xbb\x87t Nam".is_ascii());
        assert!(!b"Vi\xe1\xbb\x87t Nam".iter().all(|b| b.is_ascii()));
        assert!(!b"\xe1\xbb\x87".iter().any(|b| b.is_ascii()));

        assert!("".is_ascii());
        assert!("banana\0\u{7F}".is_ascii());
        assert!("banana\0\u{7F}".chars().all(|c| c.is_ascii()));
        assert!(!"ประเทศไทย中华Việt Nam".chars().all(|c| c.is_ascii()));
        assert!(!"ประเทศไทย中华ệ ".chars().any(|c| c.is_ascii()));
    }

    #[test]
    fn test_to_ascii_uppercase() {
        assert_eq!("url()URL()uRl()ürl".to_ascii_uppercase(), "URL()URL()URL()üRL");
        assert_eq!("hıKß".to_ascii_uppercase(), "HıKß");

        for i in 0..501 {
            let upper = if 'a' as u32 <= i && i <= 'z' as u32 { i + 'A' as u32 - 'a' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().to_ascii_uppercase(),
                       (from_u32(upper).unwrap()).to_string());
        }
    }

    #[test]
    fn test_to_ascii_lowercase() {
        assert_eq!("url()URL()uRl()Ürl".to_ascii_lowercase(), "url()url()url()Ürl");
        // Dotted capital I, Kelvin sign, Sharp S.
        assert_eq!("HİKß".to_ascii_lowercase(), "hİKß");

        for i in 0..501 {
            let lower = if 'A' as u32 <= i && i <= 'Z' as u32 { i + 'a' as u32 - 'A' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().to_ascii_lowercase(),
                       (from_u32(lower).unwrap()).to_string());
        }
    }

    #[test]
    fn test_make_ascii_lower_case() {
        macro_rules! test {
            ($from: expr, $to: expr) => {
                {
                    let mut x = $from;
                    x.make_ascii_lowercase();
                    assert_eq!(x, $to);
                }
            }
        }
        test!(b'A', b'a');
        test!(b'a', b'a');
        test!(b'!', b'!');
        test!('A', 'a');
        test!('À', 'À');
        test!('a', 'a');
        test!('!', '!');
        test!(b"H\xc3\x89".to_vec(), b"h\xc3\x89");
        test!("HİKß".to_string(), "hİKß");
    }


    #[test]
    fn test_make_ascii_upper_case() {
        macro_rules! test {
            ($from: expr, $to: expr) => {
                {
                    let mut x = $from;
                    x.make_ascii_uppercase();
                    assert_eq!(x, $to);
                }
            }
        }
        test!(b'a', b'A');
        test!(b'A', b'A');
        test!(b'!', b'!');
        test!('a', 'A');
        test!('à', 'à');
        test!('A', 'A');
        test!('!', '!');
        test!(b"h\xc3\xa9".to_vec(), b"H\xc3\xa9");
        test!("hıKß".to_string(), "HıKß");

        let mut x = "Hello".to_string();
        x[..3].make_ascii_uppercase();  // Test IndexMut on String.
        assert_eq!(x, "HELlo")
    }

    #[test]
    fn test_eq_ignore_ascii_case() {
        assert!("url()URL()uRl()Ürl".eq_ignore_ascii_case("url()url()url()Ürl"));
        assert!(!"Ürl".eq_ignore_ascii_case("ürl"));
        // Dotted capital I, Kelvin sign, Sharp S.
        assert!("HİKß".eq_ignore_ascii_case("hİKß"));
        assert!(!"İ".eq_ignore_ascii_case("i"));
        assert!(!"K".eq_ignore_ascii_case("k"));
        assert!(!"ß".eq_ignore_ascii_case("s"));

        for i in 0..501 {
            let lower = if 'A' as u32 <= i && i <= 'Z' as u32 { i + 'a' as u32 - 'A' as u32 }
                        else { i };
            assert!((from_u32(i).unwrap()).to_string().eq_ignore_ascii_case(
                    &from_u32(lower).unwrap().to_string()));
        }
    }
}
