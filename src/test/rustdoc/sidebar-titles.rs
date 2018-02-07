// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(extern_types)]

#![crate_name = "foo"]

// @has 'foo/index.html' '//*[@class="sidebar"]' 'Crate foo'
// @has - '//*[@class="sidebar-elems"]' ''

// @has 'foo/foo_mod/index.html' '//*[@class="sidebar"]' 'Module foo_mod'
// @has - '//*[@class="sidebar-elems"]' ''
pub mod foo_mod {
    pub struct __Thing {}
}

extern {
    // @has 'foo/fn.foo_ffn.html' '//*[@class="sidebar"]' 'Function foo_ffn'
    // @has - '//*[@class="sidebar-elems"]' ''
    pub fn foo_ffn();
}

// @has 'foo/fn.foo_fn.html' '//*[@class="sidebar"]' 'Function foo_fn'
// @has - '//*[@class="sidebar-elems"]' ''
pub fn foo_fn() {}

// @has 'foo/trait.FooTrait.html' '//*[@class="sidebar"]' 'Trait FooTrait'
// @has - '//*[@class="sidebar-elems"]' ''
pub trait FooTrait {}

// @has 'foo/struct.FooStruct.html' '//*[@class="sidebar"]' 'Struct FooStruct'
// @has - '//*[@class="sidebar-elems"]' ''
pub struct FooStruct;

// @has 'foo/enum.FooEnum.html' '//*[@class="sidebar"]' 'Enum FooEnum'
// @has - '//*[@class="sidebar-elems"]' ''
pub enum FooEnum {}

// @has 'foo/union.FooUnion.html' '//*[@class="sidebar"]' 'Union FooUnion'
// @has - '//*[@class="sidebar-elems"]' ''
pub union FooUnion {
    x: u8,
}

// @has 'foo/type.FooType.html' '//*[@class="sidebar"]' 'Type Definition FooType'
// @has - '//*[@class="sidebar-elems"]' ''
pub type FooType = FooStruct;

// @has 'foo/foreigntype.FooFType.html' '//*[@class="sidebar"]' 'Foreign Type FooFType'
// @has - '//*[@class="sidebar-elems"]' ''
extern {
    pub type FooFType;
}

// @has 'foo/macro.foo_macro.html' '//*[@class="sidebar"]' 'Macro foo_macro'
// @has - '//*[@class="sidebar-elems"]' ''
#[macro_export]
macro_rules! foo_macro {
    () => ();
}

// @has 'foo/primitive.bool.html' '//*[@class="sidebar"]' 'Primitive Type bool'
// @has - '//*[@class="sidebar-elems"]' ''
#[doc(primitive = "bool")]
mod bool {}

// @has 'foo/static.FOO_STATIC.html' '//*[@class="sidebar"]' 'Static FOO_STATIC'
// @has - '//*[@class="sidebar-elems"]' ''
pub static FOO_STATIC: FooStruct = FooStruct;

extern {
    // @has 'foo/static.FOO_FSTATIC.html' '//*[@class="sidebar"]' 'Static FOO_FSTATIC'
    // @has - '//*[@class="sidebar-elems"]' ''
    pub static FOO_FSTATIC: FooStruct;
}

// @has 'foo/constant.FOO_CONSTANT.html' '//*[@class="sidebar"]' 'Constant FOO_CONSTANT'
// @has - '//*[@class="sidebar-elems"]' ''
pub const FOO_CONSTANT: FooStruct = FooStruct;
