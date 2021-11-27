use object_safe::object_safe;

#[object_safe]
trait Test {
    fn process(&self, _: impl Fn(&Self) -> &Self);
}

struct TestStruct;
impl ObjectSafeTest for TestStruct {}

fn main() {}
