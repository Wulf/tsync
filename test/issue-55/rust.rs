#[tsync]
struct AppleData {
    crunchy: bool,
}

#[tsync]
struct BananaData {
    size: i32,
}

#[tsync]
struct CarrotData {
    color: String,
}

#[tsync]
#[serde(tag = "kind", content = "data")]
enum Fruit {
    Apple(AppleData),
    Banana(BananaData),
    Carrot(CarrotData),
}
