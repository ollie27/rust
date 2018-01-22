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

use std::ops::{Deref, DerefMut};

pub struct Foo;

impl Foo {
    pub fn by_explicit(self) {}
    pub fn by_ref(&self) {}
    pub fn by_mut_ref(&mut self) {}
}

// @has foo/struct.Bar.html
// @has - '//*[@class="sidebar-title"][@href="#deref-methods"]' 'Methods from Deref<Target=Foo>'
// @!has - '//*[@class="sidebar-links"]/a[@href="#method.by_explicit"]' 'by_explicit'
// @has - '//*[@class="sidebar-links"]/a[@href="#method.by_ref"]' 'by_ref'
// @!has - '//*[@class="sidebar-links"]/a[@href="#method.by_mut_ref"]' 'by_mut_ref'
pub struct Bar {
    x: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

// @has foo/struct.Baz.html
// @has - '//*[@class="sidebar-title"][@href="#deref-methods"]' 'Methods from Deref<Target=Foo>'
// @!has - '//*[@class="sidebar-links"]/a[@href="#method.by_explicit"]' 'by_explicit'
// @has - '//*[@class="sidebar-links"]/a[@href="#method.by_ref"]' 'by_ref'
// @has - '//*[@class="sidebar-links"]/a[@href="#method.by_mut_ref"]' 'by_mut_ref'
pub struct Baz {
    x: Foo,
}

impl Deref for Baz {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

impl DerefMut for Baz {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.x
    }
}

// @has foo/struct.Qux.html
// @!has - '//*[@class="sidebar-title"][@href="#deref-methods"]' 'Methods from Deref<Target=Quux>'
pub struct Qux {
    x: Quux,
}

pub struct Quux;

impl Quux {
    pub fn quux() {}
}

impl Deref for Qux {
    type Target = Quux;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}
