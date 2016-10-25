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

## A Simple Introduction to Borrowing
Borrowing is one of Rust's core features alongside its ownership checking. There
are many cases where you want to use a value temporarily, and so don't want to 
take ownership from the original binding. You can think of Rust's borrowing system
as a readers-writer lock. You can take as many read-only references to a value as 
you want as long as there are no mutable references, and you can only ever take one
mutable reference to a value at a time when there are no other references to that
value and the owning binding is mutable.
```rust
  let immut_obj = Object { value: 1 };

  let invalid_ref = &mut immut_obj; // Compiler Error: "cannot borrow immutable local variable `obj` as mutable"

  let mut obj = Object { value: 0 };

  let objref = &obj;
  let objref2 = &obj;
  let objref3 = &obj;

  let objmutref = &mut obj; // Compiler Error: "cannot borrow `obj` as mutable because it is also borrowed as immutable"
```

While an object has been borrowed you cannot make any changes to it such as 
assigning a value to one of its fields or calling a mutating method on it.
```rust
  let obj = Object { value: 0 };

  let objref = &obj;

  obj.value = 42; // Compiler Error: "cannot assign to `obj.value` because it is borrowed"
```

## Pass by Value and Pass by Reference in Rust
```rust
// Assume we have these function signatures
fn by_value(obj: Object) {
  ...
}

fn by_ref(obj: &Object) {
  ...
}
```

It becomes very important to pay attention to the way arguments are passed into
a function in Rust. Passing an argument by value moves ownership of that object
(assuming it's not copyable) into the function's scope, so you won't be able to
use it after the function call.
```rust
  let obj = Object { value: 0 };

  by_value(obj);

  let value = obj.value; // Compiler Error: "use of moved value: `obj.value`"
```

Passing an argument by reference does not take ownership of the object however, 
so the binding is still usable after the function call.
```rust
  let obj = Object { value: 0 };

  by_ref(&obj);

  let value = obj.value; // Works fine
```

References can also be passed as mutable, this has the same behavior as passing
an immutable reference into the function. When the function returns, the reference
is destroyed and the original binding can be used again. When passing a mutable
reference you should expect the internal state of the object to be changed in
some way however.

## Lifetimes
```rust
// Assume we have these functions
fn lifetime<'a>(x: &'a Object, y: &'a Object) -> &'a i32 {
  x.value
}

fn lifetimes<'a, 'b>(x: &'a Object, y: &'b Object) -> &'a i32 {
  x.value
}
```
Lifetimes in Rust are essentially just markers for the scope that an object will
live in. If two references have the same lifetime then the lifetime will be 
constrained to the shortest scope. Attempting to constrain a lifetime to a scope
that does not last long enough will cause an error.
```rust
  let obj1 = Object { value: 0 };
  let ret: &i32;
  {
    let obj2 = Object { value: 1 };

    // Lifetime of `inner_ret` is constrained to the inner block
    let inner_ret: &i32 = lifetime(&obj1, &obj2);

    // `obj2`'s lifetime is the inner scope, but `ret`'s lifetime must be as
    // long as the outer scope
    ret = lifetime(&obj1, &obj2); // Compiler Error: "`obj2` does not live long enough"
  }

```

By using different lifetime markers we can make it so certain values (like the lifetime
of the return value) are constrained only to the lifetimes that matter to it.
```rust
  let obj1 = Object { value: 0 };
  let ret: &i32;
  {
    let obj2 = Object { value: 1 };
  
    // Notice we are using the function with multiple lifetimes
    ret = lifetimes(&obj1, &obj2); // This works fine because the return lifetime is no longer constrained to `obj2`'s lifetime
  }
```

References marked by different lifetimes don't neccessarily have to live
in different scopes however.
```rust
  let obj1 = Object { value: 0 };
  let obj2 = Object { value: 1 };

  let ret: &i32 = lifetimes(&obj1, &obj2); // Works fine, both lifetimes are the same scope
```

The lifetime annotation matters in the function declaration, the return value
in the example function is bound to the lifetime of the first argument.
```rust
  let obj1 = Object { value: 0 };
  let ret: &i32;
  {
    let obj2 = Object { value: 1 };

    // The lifetime of `ret` should be the outer scope, but `obj2`'s lifetime is the inner scope
    ret = lifetimes(&obj2, &obj1); // Compiler Error: "`obj2` does not live long enough"
  }
```
