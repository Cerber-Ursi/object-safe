use object_safe::object_safe;

#[object_safe]
trait Test {
    fn assoc();
    fn generic<T: Copy>(&self, _: T) {}
    fn gets_self(&self, _: Self);
    fn returns_self(&self) -> Self;
    fn object_safe(&self) -> i32;
}

struct TestStruct;

impl Test for TestStruct {
    fn assoc() {}
    fn generic<T: Copy>(&self, _: T) {}
    fn gets_self(&self, _: Self) {}
    fn returns_self(&self) -> Self {
        TestStruct
    }
    fn object_safe(&self) -> i32 { 42 }
}

fn main() {
    let boxed: Box<dyn ObjectSafeTest> = Box::new(TestStruct);
    assert_eq!(42, boxed.object_safe());
}
