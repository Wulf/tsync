/// test/rust.rs
use tsync::tsync;
use serde::Serialize;
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

#[tsync]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
/// Book struct with camelCase field names.
struct BookCamel {
    /// Name of the book.
    name: String,
    /// Chapters of the book.
    chapters: Vec<Chapter>,
    /// Reviews of the book
    /// by users.
    user_reviews: Option<Vec<String>>,
}

/// Multiple line comments
/// are formatted on
/// separate lines
#[tsync]
struct Chapter {
    title: String,
    pages: u32,
}

#[tsync]
/// Generic struct test
struct PaginationResult<T> {
    items: Vec<T>,
    total_items: number,
}


#[tsync]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
/// Generic struct test with camelCase field names.
struct PaginationResultCamel<T> {
    items: Vec<T>,
    total_items: number,
}