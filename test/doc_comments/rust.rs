/// enum comment
#[tsync]
#[serde(rename_all = "UPPERCASE", tag = "type")]
enum EnumTest {
    /// enum property comment
    One,
    /// enum tuple comment
    Two(StructTest),
    /// enum struct comment
    Three {
        /// enum struct property comment
        id: String,
    },
}

#[tsync]
/// struct comment
struct StructTest {
    /// struct field comment
    name: String,
}

#[tsync]
/// type comment
type TypeTest = String;

#[tsync]
/// const comment
const CONST_TEST: &str = "test";

// not a doc comment test
// (notice the two forward slashes compared to for the rest of the tests)
struct NotACommentTest {
    // not a doc comment
    name: String,
}
