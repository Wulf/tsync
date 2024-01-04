use tsync::tsync;

/*
 * This test was introduced because of a bug where the "Paginated<T>" type would
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
