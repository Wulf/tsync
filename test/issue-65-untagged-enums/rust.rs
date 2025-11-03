/// test/rust.rs
use tsync::tsync;

#[derive(Serialize, Deserialize)]
#[tsync]
#[serde(untagged)]
enum Message {
    ValueOne(i32, i32),
    Value2(i32),
}

#[derive(Serialize, Deserialize)]
#[tsync]
#[serde(untagged)]
enum Message2<V, G> {
    ValueOne { a: V, b: G },
    Value2 { c: V },
    Value3(G),
    Value3(Vec<G>),
}
