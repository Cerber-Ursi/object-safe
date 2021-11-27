mod test {

use object_safe::object_safe;

#[object_safe]
pub trait Test {
    fn test<T>(&self, _: T);
    fn non_generic(&self);
}

pub struct TestStruct;
impl Test for TestStruct {
    fn test<T>(&self, _: T) {}
    fn non_generic(&self) {}
}

}

use test::{TestStruct, ObjectSafeTest};
fn main() {
    let test = TestStruct;
    test.test();
    test.non_generic();
}