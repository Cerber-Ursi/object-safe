use object_safe::object_safe;

#[object_safe(allow_sized)]
trait Test where Self: Sized {}

struct TestStruct;
impl Test for TestStruct {}

fn main() {
    let _: Box<ObjectSafeTest> = Box::new(TestStruct);
}
