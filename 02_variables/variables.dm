The `let` keyword is used to
create variables. 

```rust
let x = 4;
let name = "Jeff";
```

Variables in Rust are immutable or mutable. By default, any variable is 
immutable. This means that doing an assignment, `let y = 5;`, and then trying to
assign a value to the variable later, `y = 6;`, produces a compile error. 

To make variables mutable, use the `mut` keyword.

```rust
let mut a = 4;
a = 5; // compiles

let b = 4;
b = 5; // compile error
```

The different types of variables are best found by consulting the (Rust 
tutorial)[http://static.rust-lang.org/doc/0.8/tutorial.html#syntax-basics]

A safe way to cast variables is to use the `as` keyword.

```rust
let x = 4u;
let y = x as i32;
```
