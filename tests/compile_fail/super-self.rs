use object_safe::object_safe;

trait Super<T: ?Sized> {
    fn gen(_: T);
}

#[object_safe]
trait Test: Super<Self> {}

#[object_safe]
trait Test2: Debug + Super<Self> + Copy {}

#[object_safe]
trait Test3: Super<dyn Fn(Self)> {}

fn main() {}
