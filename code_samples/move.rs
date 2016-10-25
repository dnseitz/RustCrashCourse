#![allow(dead_code, unused_variables, unused_mut)]

struct Object {
  value: i32,
}

impl Drop for Object {
  fn drop(&mut self) {
    println!("Dropping value: {}", self.value);
  }
}

fn main() {
  let obj = Object { value: 1 };
  let obj2 = obj;

  {
    let obj3 = Object { value: 3 };
  } 
  // obj3 is deallocated at the end of this block
  // Will print "Dropping value: 3"

//  let value = obj.value; // Compiler error
}
// Only prints "Dropping value: 1" once because there
// is only one object, even though both `obj` and `obj2`
// are going out of scope
