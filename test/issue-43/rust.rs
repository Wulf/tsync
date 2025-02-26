// #[tsync]
// struct HasTuple1 {
//     foo: i32,
//     bar: Option<(String, i32)>,
// }

// #[tsync]
// struct HasTuple2 {
//     foo: i32,
//     bar: (String, i32),
// }

#[tsync]
struct IsTuple(i32, String);

#[tsync]
struct IsTupleComplex(i32, String, (String, (i32, i32)));
