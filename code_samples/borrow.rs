#![allow(dead_code, unused_variables, unused_mut)]

struct Object {
  value: i32,
}

impl Object {
  fn func(&mut self) {

  }
}

fn main() {
  let immut_obj = Object { value: 1 };

  //let invalid_ref = &mut immut_obj;

  let mut obj = Object { value: 0 };

  let objref = &obj;
  let objref2 = &obj;
  let objref3 = &obj;

  //let objrefmut = &mut obj;

  //obj.value = 42;
  //obj.func();
}
