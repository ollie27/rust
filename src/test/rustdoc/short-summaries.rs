// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]

// @has 'search-index.js' 'docs for Bar with <code>link</code>.'
// @has 'foo/index.html' '//p' 'docs for Foo as a heading.'
// @has 'foo/sidebar-items.js' 'docs for Bar with `link`.'

/// # docs for Foo as a heading.
pub trait Foo {
    // @has 'foo/trait.Foo.html' '//*[@class="docblock"]' 'docs for foo0() a*b*c* with link.'
    // @has - '//a[@href="https://example.com/"]' 'link'
    /// docs for foo0() a\*b\*c\* with [`link`].
    ///
    /// [`link`]: https://example.com/
    fn foo0() {}
    // @has 'foo/trait.Foo.html' '//*[@class="docblock"]' 'docs for foo1() with multiple lines.'
    /// docs for foo1() with
    /// multiple lines.
    fn foo1() {}
}

/// docs for Bar with [`link`].
///
/// [`link`]: https://www.rust-lang.org/
pub struct Bar;

// @has 'foo/struct.Bar.html' '//*[@class="docblock"]' 'docs for foo0() a*b*c* with link.'
// @!has - 'example.com'
// @has - '//*[@class="docblock"]' 'docs for foo1() with multiple lines.'
// @!has - '//*[@class="docblock"]' 'Read more'
impl Foo for Bar {}
