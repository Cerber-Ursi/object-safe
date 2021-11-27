use object_safe::object_safe;

#[object_safe(name = "Wrapper")]
trait Test {}

struct TestStruct;
impl Test for TestStruct {}

fn main() {
    let _: Box<dyn Wrapper> = Box::new(TestStruct);
}
