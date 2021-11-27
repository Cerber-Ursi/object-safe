#[object_safe::object_safe]
trait Test {
  fn test(&self);
}

struct TestStruct;
impl Test for TestStruct {
  fn test(&self) {
    println!("It did not crash!");
  }
}

#[test]
fn works() {
  let test = TestStruct;
  test.test();
}
