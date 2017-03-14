// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]

mod hidden {
    #[derive(Clone)]
    pub struct Foo;
}

#[doc(hidden)]
pub mod __hidden {
    pub use hidden::Foo;
}

// @has implementors/core/clone/trait.Clone.js 'Bar'
#[derive(Clone)]
pub struct Bar;

// @has foo/trait.Clone.html
// @!has - 'Foo'
// @has implementors/core/clone/trait.Clone.js
// @!has - 'Foo'
pub use std::clone::Clone;
