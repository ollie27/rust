// aux-build:duplicate-lifetime-bounds.rs

#![crate_name = "foo"]

extern crate duplicate_lifetime_bounds;

// @has foo/struct.Scope.html
// @has - '//pre' "pub struct Scope<'env> { /* fields omitted */ }"
// @!has - '//pre' "'env: 'env"
pub use duplicate_lifetime_bounds::Scope;

// @has foo/struct.Iter.html
// @has - '//pre' "pub struct Iter<'a, T> where T: 'a, { /* fields omitted */ }"
// @!has - '//pre' "T: 'a + 'a"
pub use duplicate_lifetime_bounds::Iter;

// @has foo/struct.DebugSet.html
// @has - '//pre' "pub struct DebugSet<'a, 'b> where 'b: 'a, { /* fields omitted */ }"
// @!has - '//pre' "'b: 'b"
pub use duplicate_lifetime_bounds::DebugSet;
