use object_safe::object_safe;

mod details {

    use object_safe::object_safe;

    #[object_safe]
    pub trait NonObjectSafe {
        fn generic<T: Copy>(&self, _: T);
        fn self_in(&self, _: Self);
        fn self_out(&self) -> Self;
    }

    pub struct TestStruct;
    impl NonObjectSafe for TestStruct {
        fn generic<T: Copy>(&self, _: T) {}
        fn self_in(&self, _: Self) {}
        fn self_out(&self) -> Self {
            TestStruct
        }
    }

}

use details::{ObjectSafeNonObjectSafe, TestStruct};

fn main() {
    let test = TestStruct;
    test.generic(0u32);
    test.self_in(TestStruct);
    let _ = test.self_out();
}
