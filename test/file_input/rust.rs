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
  user_reviews: Option<Vec<String>>
}

/// Multiple line comments
/// are formatted on
/// separate lines
#[tsync]
struct Chapter {
  title: String,
  pages: u32
}


#[tsync]
/// Time in UTC seconds
type UTC = usize;