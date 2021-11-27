use object_safe::object_safe;

#[object_safe]
trait SelfSized where Self: Sized {}

fn main() {}
