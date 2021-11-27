use object_safe::object_safe;

#[object_safe(allow_sized, name = Wrapper2)]
trait Test2 where Self: Sized {}

#[object_safe(name = Wrapper1, allow_sized)]
trait Test1 where Self: Sized {}


struct TestStruct;
impl Wrapper1 for TestStruct {}
impl Wrapper2 for TestStruct {}

fn main() {}
