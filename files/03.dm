[Previous](02.md)	[Next](04.md)

Before, we saw that the return type on `get_hello()` was `&str`. 

rcode code/hello_world2.rs 7 10

The `&` involves Rust pointers, which revolve around ownership. There are three
different types of pointers: `&`, `~`, and `@`. 

[Ownership](http://static.rust-lang.org/doc/0.8/tutorial.html#ownership) refers
 to which object or section of code a variable belongs to. It's used to deal 
 with variable life-cycles, including garbage collection.

 `~` refers to an owned pointer. Variables allocated with `~` are put on the
 heap. An example from the Rust tutorial demonstrates life-cycles for owned
 pointers.

 ```rust
 {
	 // an integer allocated on the heap
	 let y = ~10;
 }
 // the destructor frees the memory heap as soon as `y` goes out of scope
```

Owned pointers can be tricky. Assignment works differently than with plain
variables. Consider this code

```rust
let x = ~5;
let y = x;
println(format("{:?}", x));
println(format("{:?}", y));
```

Trying to compile gives the error 'use of moved value `x`'. Explicity writing
the types will help this error make sense.

```rust
let x: ~int = ~5;
let y: ~int = x;
println(format("{:?}", x));
println(format("{:?}", y));
```

What actually happens with the `let y: ~int = x;` is that `y` is set to point 
the same place as `x`. Since this is with owned pointers, and a variable can 
only have a single owned pointer pointing to it at a time, `x` no longer points
to anything valid. (Note, Rust doesn't have a `NULL`.)

As an aside, the following is valid code.

```rust
let x: ~int = ~5;
let y: int = *x;
println(format("{:?}", x));
println(format("{:?}", y));
```

It prints

```rust
~5
5
```

[Previous](02.md)	[Next](04.md)
