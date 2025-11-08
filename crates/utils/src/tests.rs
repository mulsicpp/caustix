use crate::Build;
use crate::Buildable;

#[derive(crate::Paramters, Default)]
struct FooBuilder {
    name: String,
    age: u32,
}

impl Build for FooBuilder {
    type Target = Foo;

    fn build(&self) -> Self::Target {
        Foo(self.name.clone(), self.age)
    }
}

#[derive(Debug)]
struct Foo(String, u32);

impl Buildable for Foo {
    type Builder<'a> = FooBuilder;
}

#[test]
pub fn test_builder() {
    let foo_builder = Foo::builder().age(32u32).name("franz");

    assert_eq!(foo_builder.name, "franz");
    assert_eq!(foo_builder.age, 32);

    let foo = foo_builder.build();

    assert_eq!(foo.0, "franz");
    assert_eq!(foo.1, 32);
}
