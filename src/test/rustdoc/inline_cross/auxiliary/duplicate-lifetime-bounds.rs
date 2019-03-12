pub struct Scope<'env> {
    _marker: std::marker::PhantomData<&'env mut &'env ()>,
}

pub struct Iter<'a, T: 'a> {
    _marker: std::marker::PhantomData<&'a T>,
}

struct DebugInner<'a, 'b: 'a> {
    fmt: &'a mut std::fmt::Formatter<'b>,
}

pub struct DebugSet<'a, 'b: 'a> {
    inner: DebugInner<'a, 'b>,
}
