#Syntax - Loops, Macros, and Matching#
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

The different types of variables are best found by consulting the [Rust 
tutorial](http://static.rust-lang.org/doc/0.8/tutorial.html#syntax-basics).

A safe way to cast variables is to use the `as` keyword.

```rust
let x = 4u; // x is an unsigned integer
let y = x as i32; // y is x as a 32-bit integer
```

Another example will help clarify this and the previous section. This code 
recursively computes the number of steps in a number's Collatz sequence.

rcode code/collatz.rs

There are one or two new things here. The first is the `format!`. The `!` means
that it is a macro, and `format!` is used to print data in human readable form.
It's similar to C's `printf`.

The second new bit is specific to Rust's syntax. `if` statements don't have
parenthesis around their conditionals. However, each `if` statement must have a
body, and that body must be surrounded by braces.

Rust also features a `match` statement. It's an alternative to `if` statements 
and is considered "prettier". The last `if-else` group can be replaced with a 
match statement.

rcode code/collatz2.rs

The `_` is similar to a `switch` statement's `default` in Java. If nothing
matches the variable, the statement falls through to the `_` case. The Rust
compiler requires an execution path for each possible outcome, and will 
complain if the match conditions are not exhaustive. It's not an oracle though,
 and will throw an error if the `_` is replaced with `1`. (It's better with 
 booleans.)

The following `if-else` statement and `match` statement are equivalent.
```rust
if doSomething
{
	doSomething();
}
else
{
	doNothing();
}

match doSomething
{
	true => { doSomething(); }
	false => { doNothing(); }
	// no _ is needed. The compiler can figure out that this match
	// statement is exhaustive.
}
```

One additional thing to note about `match` statements is that the code for a
case must be surrounded in braces, even if it's empty.

```rust
match x
{
	4 => { print("It's four"); }
	// 5 => print("It's five"); // not valid
	_ => {;}
}
```

Rust vectors (i.e. arrays) are made using `[` and `]`. Vectors made this way 
are put on the stack.

rcode code/array.rs 3 4

Arrays can also be made using `let array = [x, ..N]`. This makes an array of
size `N` and sets each value to `x`.

rcode code/array.rs 6 6

Rust has several different types of loops. All Rust loops need to have their
code surrounded in braces (`{`, `}`). All Rust loops work with the `break`
and `continue` keywords, whose meanings are the same as in C++.

The most basic loop is the `loop` keyword. It loops until a `break` is 
encountered.

rcode code/loops.rs 4 8

rcode code/loops.rs 11 19

Rust also has a `while` loop. Its syntax is almost exactly the same as C, C++, 
or Java. The previous loops can easily be rewritten using `while` loops.

rcode code/loops.rs 21 24

rcode code/loops.rs 27 32

Rust `for` loops are different from those in C, C++, and Java. They're more
akin to Java's `for-each` loop. `for` loops work by using an iterator.

We need to use an iterator to loop over elements in a vector. This shows that
vectors come with some additional functionality that other languages' arrays
lack.

rcode code/loops.rs 34 39

There's a `range(a, b)` function that can be used to loop over a sequential 
set of numbers, `[a,b)`.

rcode code/loops.rs 41 45

Here is a Rust quine combining loops, variables, and arrays. It should 
provide good examples of Rust syntax.

rcode code/quine.rs
