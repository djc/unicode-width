// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Determine displayed width of `char` and `str` types according to
//! [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
//! rules.
//!
//! ```rust
//! extern crate unicode_width;
//!
//! use unicode_width::UnicodeWidthStr;
//!
//! fn main() {
//!     let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
//!     let width = UnicodeWidthStr::width(teststr);
//!     println!("{}", teststr);
//!     println!("The above string is {} columns wide.", width);
//!     let width = teststr.width_cjk();
//!     println!("The above string is {} columns wide (CJK).", width);
//! }
//! ```
//!
//! # crates.io
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unicode_width = "0.0.1"
//! ```

#![deny(missing_docs, unsafe_code)]
#![feature(no_std, core)]
#![no_std]

extern crate core;

#[cfg(test)]
#[macro_use]
extern crate std;

use core::prelude::*;

use tables::charwidth as cw;
pub use tables::UNICODE_VERSION;

mod tables;

/// Methods for determining displayed width of Unicode characters.
#[allow(missing_docs)]
pub trait UnicodeWidthChar {
    fn width(self) -> Option<usize>;
    fn width_cjk(self) -> Option<usize>;
}

impl UnicodeWidthChar for char {
    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character other than `'\x00'`.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for non-CJK
    /// contexts, or when the context cannot be reliably determined.
    fn width(self) -> Option<usize> { cw::width(self, false) }

    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character other than `'\x00'`.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 columns wide. This is consistent with the recommendations for
    /// CJK contexts.
    fn width_cjk(self) -> Option<usize> { cw::width(self, true) }
}

/// Methods for determining displayed width of Unicode strings.
#[allow(missing_docs)]
pub trait UnicodeWidthStr {
    fn width<'a>(&'a self) -> usize;
    fn width_cjk<'a>(&'a self) -> usize;
}

impl UnicodeWidthStr for str {
    /// Returns the string's displayed width in columns.
    ///
    /// Control characters are treated as having zero width.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for
    /// non-CJK contexts, or when the context cannot be reliably determined.
    fn width(&self) -> usize {
        self.chars().map(|c| cw::width(c, false).unwrap_or(0)).sum()
    }

    /// Returns the string's displayed width in columns.
    ///
    /// Control characters are treated as having zero width.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 column wide. This is consistent with the recommendations for
    /// CJK contexts.
    fn width_cjk(&self) -> usize {
        self.chars().map(|c| cw::width(c, true).unwrap_or(0)).sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_str() {
        use super::UnicodeWidthStr;

        assert_eq!(UnicodeWidthStr::width("ｈｅｌｌｏ"), 10);
        assert_eq!("ｈｅｌｌｏ".width_cjk(), 10);
        assert_eq!(UnicodeWidthStr::width("\0\0\0\x01\x01"), 0);
        assert_eq!("\0\0\0\x01\x01".width_cjk(), 0);
        assert_eq!(UnicodeWidthStr::width(""), 0);
        assert_eq!("".width_cjk(), 0);
        assert_eq!(UnicodeWidthStr::width("\u{2081}\u{2082}\u{2083}\u{2084}"), 4);
        assert_eq!("\u{2081}\u{2082}\u{2083}\u{2084}".width_cjk(), 8);
    }

    #[test]
    fn test_char() {
        use super::UnicodeWidthChar;
        use core::option::Option::{Some, None};

        assert_eq!(UnicodeWidthChar::width('ｈ'), Some(2));
        assert_eq!('ｈ'.width_cjk(), Some(2));
        assert_eq!(UnicodeWidthChar::width('\x00'), Some(0));
        assert_eq!('\x00'.width_cjk(), Some(0));
        assert_eq!(UnicodeWidthChar::width('\x01'), None);
        assert_eq!('\x01'.width_cjk(), None);
        assert_eq!(UnicodeWidthChar::width('\u{2081}'), Some(1));
        assert_eq!('\u{2081}'.width_cjk(), Some(2));
    }
}