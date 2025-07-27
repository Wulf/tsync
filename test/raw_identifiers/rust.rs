#[tsync]
struct RawIdentifierStruct {
    r#type: String,
    r#async: i32,
    r#loop: bool,
    normal_field: String,
}

#[tsync]
#[serde(rename_all = "camelCase")]
struct RawIdentifierCamelCase {
    r#type: String,
    r#const: u32,
    regular_field: String,
}

#[tsync]
enum RawIdentifierEnum {
    r#type,
    r#async,
    r#match,
    NormalVariant,
}

#[tsync]
#[serde(rename_all = "UPPERCASE")]
enum RawIdentifierEnumUppercase {
    r#type,
    r#const,
    NormalVariant,
}

#[tsync]
#[repr(u8)]
enum RawIdentifierNumericEnum {
    r#type = 1,
    r#async = 2,
    NormalVariant = 3,
}

#[tsync]
#[serde(tag = "kind")]
enum RawIdentifierTaggedEnum {
    r#type { value: String },
    r#async { count: u32 },
    NormalVariant { data: bool },
}

#[tsync]
type r#type = String;

#[tsync]
type r#async = Vec<i32>;
