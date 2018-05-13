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

// @has foo/fn.foo.html
// @has - '//*[@class="rust fn"]' "_: &(ToString + 'static)"
pub fn foo(_: &(ToString + 'static)) {}

// @has foo/fn.bar.html
// @has - '//*[@class="rust fn"]' "_: *const (ToString + 'static)"
pub fn bar(_: *const (ToString + 'static)) {}

// @has foo/fn.baz.html
// @has - '//*[@class="rust fn"]' "_: *mut (ToString + 'static)"
pub fn baz(_: *mut (ToString + 'static)) {}

// @has foo/fn.qux.html
// @has - '//*[@class="rust fn"]' "-> &(impl ToString + 'static)"
pub fn qux(x: &u8) -> &(impl ToString + 'static) {
    x
}

// @has foo/fn.quux.html
// @has - '//*[@class="rust fn"]' "-> *const (impl ToString + 'static)"
pub fn quux(x: &u8) -> *const (impl ToString + 'static) {
    x
}

// @has foo/fn.quuz.html
// @has - '//*[@class="rust fn"]' "-> *mut (impl ToString + 'static)"
pub fn quuz(x: &mut u8) -> *mut (impl ToString + 'static) {
    x
}
