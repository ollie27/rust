#![crate_name = "foo"]

use std::ops::Deref;

pub struct Foo;

impl Deref for Foo {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        "Foo"
    }
}

// @has 'foo/struct.Foo.html'
// @has - '//*[@id="deref-methods"]' 'Methods from Deref<Target = str>'
// @has - '//*[@id="method.chars"]' 'pub fn chars(&self) -> Chars'
// @has - '//*[@id="method.to_uppercase"]' 'pub fn to_uppercase(&self) -> String'
