pub trait MyTrait2<'a, 'b> {}

pub type MyBoxedTraitObject2 = Box<dyn for<'c, 'd> MyTrait2<'c, 'd> + Send + 'static>;
