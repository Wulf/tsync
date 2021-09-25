/// test/rust.rs
use tsync::tsync;

/// Doc comments are preserved too!
#[tsync]
struct Book {
  name: String,
  chapters: Vec<Chapter>,
  user_reviews: Option<Vec<String>>
}

#[tsync]
struct Chapter {
  title: String,
  pages: u32
}


#[tsync]
/// Time in UTC seconds
type UTC = usize;