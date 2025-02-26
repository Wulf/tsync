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
    /// Note: this test case will not be included in the generated typescript,
    /// because it is a tuple variant
    Custom(CustomTopping),
    /// two custom toppings
    /// Note: this test case will not be included in the generated typescript,
    /// because it is a tuple variant
    CustomTwo(CustomTopping, CustomTopping),
}

/// Adjacently tagged enums have a key-value pair
/// that discrimate which variant it belongs to, and
/// can support tuple variants
#[tsync]
#[serde(tag = "type", content = "value")]
enum AdjacentTopping {
    /// Tasty!
    /// Not vegetarian
    Pepperoni,
    /// For cheese lovers
    ExtraCheese { kind: String },
    /// Custom toppings
    /// May expire soon
    Custom(CustomTopping),
    /// two custom toppings
    /// Note: this test case is specifically for specifying a tuple of types
    CustomTwo(CustomTopping, CustomTopping),
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
    /// two custom toppings
    /// Note: this test case is specifically for specifying a tuple of types
    CustomTwo(CustomTopping, CustomTopping),
}

#[tsync]
struct CustomTopping {
    name: String,
    expires_in: NaiveDateTime,
}

#[tsync]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomToppingCamel {
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

// Regression: if "serde(tag)" is specified, don't output a string.
#[tsync]
#[serde(tag = "type")]
enum Tagged {
    Test, // this should be { type: "Test" } in the TypeScript (not just the string "Test")
}
