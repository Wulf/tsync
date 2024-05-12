use tsync::tsync;

/// Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
#[derive(Serialize_repr)]
#[tsync]
enum Foo {
    Bar,       // 0
    Baz = 123, // 123
    Quux,      // 124
}
