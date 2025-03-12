use tsync::tsync;

/*
 * This test was introduced because of a bug where the "Paginated<Folder>" type would
 * be converted to "Paginated" without the generic type.
 */

#[tsync]
struct Folder {
    name: String,
    children: Paginated<Folder>,
}

#[tsync]
struct Paginated<T> {
    data: Vec<T>,
    page: u32,
    total_pages: u32,
}

#[tsync]
struct Flatten<T> {
    name: String,
    #[serde(flatten)]
    data: Vec<T>,
}

/**
 * Test enum represenations w/ generics
 */

#[tsync]
enum ExternalEnum<T, U> {
    Bar(T),
    Waz(U),
}

#[tsync]
#[serde(tag = "type", content = "value")]
enum AdjacentEnum<T, U> {
    Bar(T),
    Waz(U),
}

#[tsync]
#[serde(tag = "type")]
enum InternalEnum<T, U> {
    Bar { value: T, alias: String },
    Waz(U),
}
