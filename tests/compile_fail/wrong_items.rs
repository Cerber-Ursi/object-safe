use object_safe::object_safe;

#[object_safe]
fn test() {}

#[object_safe]
struct Test;

#[object_safe]
type TestType = Test;

fn main() {}