use object_safe::object_safe;

    #[object_safe]
    pub trait Empty {}

    pub struct TestStruct;
    impl ObjectSafeEmpty for TestStruct {}

fn main() {}
