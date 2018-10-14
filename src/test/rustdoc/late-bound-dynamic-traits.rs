// aux-build:late-bound-dynamic-traits.rs

#![crate_name = "foo"]

pub trait MyTrait<'a, 'b> {}

// @has 'foo/type.MyBoxedTraitObject.html' '//pre' "Box<dyn for<'c, 'd> MyTrait<'c, 'd> + Send + 'static>"
pub type MyBoxedTraitObject = Box<dyn for<'c, 'd> MyTrait<'c, 'd> + Send + 'static>;

extern crate late_bound_dynamic_traits;

// @has 'foo/type.MyBoxedTraitObject2.html' '//pre' "Box<dyn for<'c, 'd> MyTrait2<'c, 'd> + Send + 'static>"
pub use late_bound_dynamic_traits::{MyTrait2, MyBoxedTraitObject2};
