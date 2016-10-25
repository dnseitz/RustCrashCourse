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
```


