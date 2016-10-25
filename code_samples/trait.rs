#![allow(dead_code, unused_variables, unused_mut)]

struct Object {
  value: i32,
}

trait MyTrait {
  fn a_method(&self) -> i32;
}

trait SubTrait : MyTrait {
  fn times_two(&self) -> i32 { self.a_method() * 2 }
}

impl MyTrait for Object {
  fn a_method(&self) -> i32 {
    self.value
  }
}

impl MyTrait for i32 {
  fn a_method(&self) -> i32 {
    *self
  }
}

fn main() {
  let obj = Object { value: 42 };

  println!("{}", call_trait(obj));

  println!("{}", call_trait(100));
}

fn call_trait<T: MyTrait>(obj: T) -> i32 {
  obj.a_method()
}
