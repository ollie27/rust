// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]

use std::ops::Deref;

// @has foo/struct.Foo.html
// @has - '//*[@class="sidebar-title"][@href="#deref-methods"]' 'Methods from Deref<Target=Bar>'
// @has - '//*[@class="sidebar-links"]/a[@href="#method.bar_method"]' 'bar_method'
pub struct Foo {
    x: Bar,
}

pub struct Bar;

impl Bar {
    pub fn bar_method(&self) {}
}

impl Deref for Foo {
    type Target = Bar;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

// @has foo/struct.Qux.html
// @!has - '//*[@class="sidebar-title"][@href="#deref-methods"]' 'Methods from Deref<Target=Quux>'
pub struct Qux {
    x: Quux,
}

#[derive(Clone)]
pub struct Quux;

impl Deref for Qux {
    type Target = Quux;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}
