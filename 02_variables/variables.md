The Rust language is centered around safety. The `let` keyword is used to
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
a = 5; // compiles okay

let b = 4;
b = 5; // compile error
```
