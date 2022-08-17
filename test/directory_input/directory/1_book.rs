/// test/rust.rs
use tsync::tsync;

/// Doc comments are preserved too!
#[tsync]
struct Book {
    /// Name of the book.
    name: String,
    /// Chapters of the book.
    chapters: Vec<Chapter>,
    /// Reviews of the book
    /// by users.
    user_reviews: Option<Vec<String>>,
}
