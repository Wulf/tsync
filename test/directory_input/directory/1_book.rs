/// test/rust.rs
use tsync::tsync;

/// Doc comments are preserved too!
#[tsync]
struct Book {
    name: String,
    chapters: Vec<Chapter>,
    user_reviews: Option<Vec<String>>,
}
