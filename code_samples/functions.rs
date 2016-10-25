#![allow(dead_code, unused_variables, unused_mut)]

struct Object {
  value: i32,
}

fn main() {
  let obj = Object { value: 0 };

  by_ref(&obj);

  //by_value(obj);

  let value = obj.value;
}

fn by_value(obj: Object) {

}

fn by_ref(obj: &Object) {

}
