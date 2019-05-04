#![feature(optin_builtin_traits)]

pub auto trait Bar {}

impl<T> Bar for T {}

// @has blanket_auto/struct.Foo.html
// @!has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]' 'Bar'
// @has - '//*[@id="blanket-implementations-list"]/*[@class="impl"]//code' 'impl<T> Bar for T'
pub struct Foo;
