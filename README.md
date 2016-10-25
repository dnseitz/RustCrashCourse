# Rust Crash Course

## Move vs Copy Semantics

```rust
// Assume we use this custom struct
struct Object {
  value: i32,
}
```
By default, Rust uses move semantics on all user defined types. This is the 
core of Rust's ownership and borrowing system. When creating a variable binding 
in Rust that variable binding takes ownership of whatever object it has created.
```rust
  // obj is a variable binding, which binds the name obj to the actual Object created
  // obj is the owner of the created Object
  let obj = Object { value: 1 };
```

While a variable binding holds ownership over an object, it can do whatever it
wants with it. The owner handles all memory management for the object, when the
owning variable binding goes out of scope the object it binds to is deallocated.
```rust
  {
    let obj = Object { value: 1 };
  }
  // obj is deallocated at the end of this block
  // See move.rs in the code_samples directory
  // for more information
```

When one variable binding takes ownership from another, the value is 'moved' out
of the initial binding. It becomes an error to try and use a variable binding that
has had its value moved.
```rust
  let obj = Object { value: 1 };
  
  // The created Object is moved out of `obj` and into `obj2`
  let obj2 = obj;

  // It becomes an error to try and use the `obj` binding
  let value = obj.value; // Compiler error: "use of moved value: `obj.value`"
```

Not all of Rust's values use move semantics however. You may have encountered code 
similar to the following:
```rust
  let num: i32 = 42;

  let num2: i32 = num;

  println!("{}", num); // Not an error
```

One might think that this violates Rust's ownership system, we're assigning `num2`
the value stored in `num`, so shouldn't num be unusable? This behavior is because
Rust also uses copy semantics on many of its types including all of its primitive
types. Instead of moving a value from one variable binding to another, it copies 
the value and assigns the copy to the new binding. This means that both variable
bindings store independent values.
```rust
  let mut num: i32 = 0;

  let mut num2: i32 = num;

  num = 42;

  println!("num={}, num2={}", num, num2); // Prints "num=42, num2=0"
```

User defined types can choose to use copy semantics over move semantics with the
`#[derive(Copy, Clone)]` attribute. This can only be done on user defined types if
all of the type's components implement `Copy`. 
```rust
struct Movable;

// Works fine
#[derive(Copy, Clone)]
struct Copyable;

// Compiler error: "field `value` does not implement `Copy`"
#[derive(Copy, Clone)] 
struct UnCopyable {
  value: Movable,
}
```

