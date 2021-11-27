use object_safe::object_safe;

#[object_safe]
trait Test {
    fn assoc();
    fn generic<T: Copy>(&self, _: T) {}
    fn gets_self(&self, _: Self);
    fn returns_self(&self) -> Self;
    fn object_safe(&self);
}

struct TestStruct;

impl Test for TestStruct {
    fn assoc() {}
    fn generic<T: Copy>(&self, _: T) {}
    fn gets_self(&self, _: Self) {}
    fn returns_self(&self) -> Self {
        TestStruct
    }
    fn object_safe(&self) {}
}

fn main() {
    let boxed: Box<dyn ObjectSafeTest> = Box::new(TestStruct);
    boxed.object_safe();
}
