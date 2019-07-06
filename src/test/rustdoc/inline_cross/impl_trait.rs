// aux-build:impl_trait_aux.rs

extern crate impl_trait_aux;

// @has impl_trait/fn.func.html
// @has - '//pre[@class="rust fn"]' '(_x: impl '
// @has - '//pre[@class="rust fn"]' 'Clone'
// @has - '//pre[@class="rust fn"]' 'Into'
// @has - '//pre[@class="rust fn"]' "'a"
// @has - '//pre[@class="rust fn"]' "pub fn func<'a>(_x: impl Clone + Into<Vec<u8>> + 'a)"
// @!has - '//pre[@class="rust fn"]' 'where'
pub use impl_trait_aux::func;

// @has impl_trait/struct.Foo.html
// @has - '//code[@id="method.v"]' '(_x: impl '
// @has - '//code[@id="method.v"]' 'Clone'
// @has - '//code[@id="method.v"]' 'Into'
// @has - '//code[@id="method.v"]' "'a"
// @has - '//code[@id="method.v"]' "pub fn method<'a>(_x: impl Clone + Into<Vec<u8>> + 'a)"
// @!has - '//code[@id="method.v"]' 'where'
pub use impl_trait_aux::Foo;

// @has impl_trait/fn.func2.html
// @has - '//pre[@class="rust fn"]' "pub fn func2(_x: impl Fn(u8) -> u16, _y: impl Fn(u8) -> u16)"
// @!has - '//pre[@class="rust fn"]' 'where'
pub use impl_trait_aux::func2;

// @has impl_trait/fn.func3.html
// @has - '//pre[@class="rust fn"]' "pub fn func3(_x: impl Iterator<Item = u8>)"
// @!has - '//pre[@class="rust fn"]' 'where'
pub use impl_trait_aux::func3;
