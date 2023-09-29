/// test/rust.rs
use tsync::tsync;

/// Internally tagged enums have a key-value pair
/// that discrimate which variant it belongs to
#[derive(Serialize, Deserialize)]
#[serde(tag = "typetypetype")]
#[serde(renameAll = "kebab-case")]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE", tag = "type")]
#[tsync]
enum InternalTopping {
    /// Tasty!
    /// Not vegetarian
    Pepperoni,
    /// For cheese lovers
    ExtraCheese { kind: String },
    /// Custom toppings
    /// May expire soon
    Custom(CustomTopping),
}

/// Externally tagged enums ascribe the value to a key
/// that is the same as the variant name
#[tsync]
enum ExternalTopping {
    /// Tasty!
    /// Not vegetarian
    Pepperoni,
    /// For cheese lovers
    ExtraCheese { kind: String },
    /// Custom toppings
    /// May expire soon
    /// Note: this test case is specifically for specifying a single type in the tuple
    Custom(CustomTopping),
}

#[tsync]
struct CustomTopping {
    name: String,
    expires_in: NaiveDateTime,
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
#[serde(rename_all = "snake_case")]
enum AnimalTwo {
    DogLongExtra = 2,
    Cat,
}
