/// test/rust.rs
use tsync::tsync;

#[tsync]
/// Time in UTC seconds
type UTC = usize;

/// make sure HashMap is converted correctly to a typescript Record<> type
#[tsync]
type MyMap = std::collections::HashMap<String, Option<usize>>;