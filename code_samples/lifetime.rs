#![allow(dead_code, unused_variables, unused_mut)]

struct Object {
  value: i32,
}

fn main() {
  let obj1 = Object { value: 0 };
  let ret: &i32;
  {
    let obj2 = Object { value: 1 };
    
    let inner_ret: &i32 = lifetime(&obj1, &obj2);
    //ret = lifetime(&obj1, &obj2);
    ret = lifetimes(&obj2, &obj1);

    ret = lifetimes(&obj1, &obj2);
  }

  println!("{}", ret);
}

fn lifetime<'a>(x: &'a Object, y: &'a Object) -> &'a i32 {
  &x.value
}

fn lifetimes<'a, 'b>(x: &'a Object, y: &'b Object) -> &'a i32 {
  &x.value
}
