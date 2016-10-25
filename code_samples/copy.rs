#![allow(dead_code, unused_variables, unused_mut)]

struct Movable;

#[derive(Copy, Clone)]
struct Copyable;

//#[derive(Copy, Clone)]
//struct UnCopyable {
//  value: Movable,
//}

fn main() {
  let num: i32 = 42;

  let num2: i32 = num;

  println!("{}", num);

  {
    let mut num: i32 = 0;

    let mut num2: i32 = num;

    num = 42;

    println!("num={}, num2={}", num, num2);
  }
}
