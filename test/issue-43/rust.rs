#[tsync]
struct HasTuple {
    foo: i32,
    bar: Option<(String, i32)>,
    baz: (String, i32),
    zip: (i32, String, (String, (i32, i32))),
    qux: (Option<String>, (i32, String)),
    ping: (i32, String, Option<(String, (i32, i32))>),
    pong: Option<(i32, String, Option<(String, Option<(i32, i32)>)>)>,
}

#[tsync]
struct IsTuple(i32, String);

#[tsync]
struct IsTupleComplex(i32, String, (String, (i32, i32)));
