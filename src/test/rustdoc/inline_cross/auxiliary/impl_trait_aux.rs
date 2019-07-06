pub fn func<'a>(_x: impl Clone + Into<Vec<u8>> + 'a) {}

pub struct Foo;

impl Foo {
    pub fn method<'a>(_x: impl Clone + Into<Vec<u8>> + 'a) {}
}

pub fn func2(_x: impl Fn(u8) -> u16, _y: impl Fn(u8) -> u16) {}

pub fn func3(_x: impl Iterator<Item = u8>) {}
