use tsync::tsync;

#[tsync]
struct Chapter {
    title: String,
    pages: u32,
}
