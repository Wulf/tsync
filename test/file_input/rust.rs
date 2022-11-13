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

/// Multiple line comments
/// are formatted on
/// separate lines
#[tsync]
struct Chapter {
    title: String,
    pages: u32,
}

#[tsync]
/// Time in UTC seconds
type UTC = usize;

#[tsync]
/// Generic struct test
struct PaginationResult<T> {
    items: Vec<T>,
    total_items: number,
}

#[tsync]
/// Test integer
const CONST_TEST_1: i32 = 0;
#[tsync]
/// Shouldn't compile but should convert
const CONST_TEST_2: i32 = 0.0;
#[tsync]
/// Valid Rust but not valid typescript would be misleading if it made it into normal string ?
const CONST_TEST_3: &'static [u8] = b"Hello";
#[tsync]
/// Test serde_json
const SERDE_JSON_1: serde_json::Value = serde_json::json!({ "a": "b" });
#[tsync]
const SERDE_JSON_2: serde_json::Value = json!({ "a": "b" });

/// Variants should to discriminated unions
/// The last serde/attribute combo matching the tag should be taken
#[derive(Serialize, Deserialize)]
#[serde(tag = "somethingelse")]
#[serde(renameAll = "kebab-case")]
#[serde(tag = "last_precedent")]
#[tsync]
enum Message {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// The default enum conversion uses external tagging
#[tsync]
enum ExternalMessage {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// All Unit Enums go to union of constant strings
/// even if have explicit numeric annotations
/// There is no case renaming on default
#[tsync]
enum Animal {
    Dog,
    Cat,
}
#[tsync]
#[serde(renameAll = "snake_case")]
enum AnimalTwo {
    DogLongExtra = 2,
    Cat,
}

/// Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
#[derive(Serialize_repr)]
#[tsync]
enum Foo {
    Bar,       // 0
    Baz = 123, // 123
    Quux,      // 124
}
