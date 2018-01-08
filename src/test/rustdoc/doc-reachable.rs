// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:doc-reachable-1.rs
// build-aux-docs
// ignore-cross-compile

#![crate_name = "foo"]

extern crate doc_reachable_1;

use doc_reachable_1::parse::ParseSess;

pub fn modify(sess: &ParseSess) {}

// @has 'foo/fn.modify.html' '//a/@href' '../doc_reachable_1/parse/struct.ParseSess.html'
