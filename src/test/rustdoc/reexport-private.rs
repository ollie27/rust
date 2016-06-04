// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="foo"]

// @has 'foo/index.html'
// @has 'foo/sidebar-items.js'

// @!has 'foo/a/index.html'
// @!has 'foo/a/sidebar-items.js'
mod a {
    // @!has 'foo/a/b/index.html'
    // @!has 'foo/a/b/sidebar-items.js'
    pub mod b {
        // @!has 'foo/a/b/struct.Foo.html'
        pub struct Foo;
    }
}

#[doc(hidden)]
pub mod hidden {
    pub use a::b;
}