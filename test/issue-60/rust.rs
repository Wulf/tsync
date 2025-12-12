#[derive(Serialize_repr)]
#[repr(u16)]
#[tsync]
pub enum KeyCode {
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
}

#[derive(Serialize_repr)]
#[repr(u16)]
#[tsync]
pub enum KeyCodeHex {
    Space = 0x0020,
    Apostrophe = 0x0027,
    Comma = 0x002c,
    Minus = 0x002d,
}

#[derive(Serialize_repr)]
#[repr(u16)]
#[tsync]
pub enum KeyCodeOct {
    Space = 0o40,
    Apostrophe = 0o47,
    Comma = 0o54,
    Minus = 0o55,
}

#[derive(Serialize_repr)]
#[repr(u16)]
#[tsync]
pub enum KeyCodeBin {
    Space = 0b100000,
    Apostrophe = 0b100111,
    Comma = 0b101100,
    Minus = 0b101101,
}
