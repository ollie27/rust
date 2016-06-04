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

// @!has foo/a/index.html
// @!has foo/a/sidebar-items.js
mod a {
    // @has 'foo/a/struct.Foo.html'
    // @has - '//a[@href="../../foo/struct.Bar.html"]' \
    //     '../../foo/struct.Bar.html'
    pub struct Foo;
}

// @has 'foo/index.html'
// @!has - 'Foo'
// @has 'foo/struct.Bar.html'
// @!has - 'Foo'
pub use a::Foo as Bar;
