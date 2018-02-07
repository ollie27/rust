// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(conservative_impl_trait)]

#![crate_name = "foo"]

// @has foo/fn.foo.html
// @has - '//*[@class="rust fn"]' "_: &(ToString + 'static)"
pub fn foo(_: &(ToString + 'static)) {}

// @has foo/fn.foo_ptr.html
// @has - '//*[@class="rust fn"]' "where F: FnOnce(&i32) -> *const (ToString + Send) + Sync"
pub fn foo_ptr<F>(f: F) where F: FnOnce(&i32) -> *const (ToString + Send) + Sync {}

// @has foo/fn.foo_ref.html
// @has - '//*[@class="rust fn"]' "where F: FnOnce(&i32) -> &(ToString + Send) + Sync"
pub fn foo_ref<F>(f: F) where F: FnOnce(&i32) -> &(ToString + Send) + Sync {}

// @has foo/fn.foo_dyn.html
// @has - '//*[@class="rust fn"]' "where F: FnOnce(&i32) -> (ToString + Send) + Sync"
pub fn foo_dyn<F>(f: F) where F: FnOnce(&i32) -> (ToString + Send) + Sync {}

// @has foo/fn.foo_fn_ptr.html
// @has - '//*[@class="rust fn"]' "where F: FnOnce(&i32) -> fn() -> (ToString + Send) + Sync"
pub fn foo_fn_ptr<F>(f: F) where F: FnOnce(&i32) -> fn() -> (ToString + Send) + Sync {}

// @has foo/fn.bar_ref.html
// @has - '//*[@class="rust fn"]' "(x: &'a (ToString + Send)) -> &'a (ToString + Send)"
pub fn bar_ref<'a>(x: &'a (ToString + Send)) -> &'a (ToString + Send) {
    x
}

// @has foo/fn.bar_ptr.html
// @has - '//*[@class="rust fn"]' "(x: *const (ToString + Send)) -> *const (ToString + Send)"
pub fn bar_ptr(x: *const (ToString + Send)) -> *const (ToString + Send) {
    x
}

// @has foo/fn.bar_ref_impl.html
// @has - '//*[@class="rust fn"]' "() -> &'a (impl ToString + Send)"
pub fn bar_ref_impl<'a>() -> &'a (impl ToString + Send) {
    &0u8
}

// @has foo/fn.bar_ptr_impl.html
// @has - '//*[@class="rust fn"]' "() -> *const (impl ToString + Send)"
pub fn bar_ptr_impl() -> *const (impl ToString + Send) {
    &0u8
}

// @has foo/fn.bar_impl.html
// @has - '//*[@class="rust fn"]' "() -> impl ToString + Send"
pub fn bar_impl() -> impl ToString + Send {
    0u8
}

// @has foo/fn.bar_fn_ptr.html
// @has - '//*[@class="rust fn"]' "(x: fn() -> (ToString + Send)) -> fn() -> (ToString + Send)"
pub fn bar_fn_ptr(x: fn() -> (ToString + Send)) -> fn() -> (ToString + Send) {
    x
}

// @has foo/fn.bar_ref_fn_trait.html
// @has - '//*[@class="rust fn"]' "( x: &(Fn() -> (ToString + Send) + Sync)) -> &(Fn() -> (ToString + Send) + Sync)"
pub fn bar_ref_fn_trait(x: &(Fn() -> (ToString + Send) + Sync)) -> &(Fn() -> (ToString + Send) + Sync) {
    x
}

// @has foo/fn.bar_ptr_fn_trait.html
// @has - '//*[@class="rust fn"]' "( x: *const (Fn() -> (ToString + Send) + Sync)) -> *const (Fn() -> (ToString + Send) + Sync)"
pub fn bar_ptr_fn_trait(x: *const (Fn() -> (ToString + Send) + Sync)) -> *const (Fn() -> (ToString + Send) + Sync) {
    x
}
