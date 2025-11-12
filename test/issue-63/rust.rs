#[tsync]
struct TestStruct {
    size: Box<i32>,
}

#[tsync]
const v: Box<i32> = 23;

#[tsync]
struct TestStruct2 {
    inner: Box<TestStruct>,
    inner_unboxed: TestStruct,
}
