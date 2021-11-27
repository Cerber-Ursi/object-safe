use object_safe::object_safe;

#[object_safe]
trait Test {
    const VAR: i32;
    fn generic<T>(&self, _: T);
    fn associated();
    fn takes_self(&self, _: Self);
    fn returns_self(&self) -> Self;
}

struct TestStruct;
impl ObjectSafeTest for TestStruct {}

fn main() {}