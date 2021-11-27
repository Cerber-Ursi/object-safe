mod traits {

    #[object_safe::object_safe]
    pub trait Test {
        fn test(&self);
    }

    pub struct TestStruct;

    impl Test for TestStruct {
        fn test(&self) {
            println!("It did not crash!");
        }
    }
}

use traits::{ObjectSafeTest, TestStruct};

#[test]
fn works() {
    let test = TestStruct;
    test.test();
}
