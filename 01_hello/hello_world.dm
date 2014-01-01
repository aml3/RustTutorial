Rust's syntax is heavily influenced by C/C++. For example, scope is determined 
by `{` and `}`, and statements end with a `;`. Like other C-family languages, 
whitespace (e.g. tabs and spaces) is ignored by the compiler. 

Unlike C/C++, we create a function using the `fn` keyword.

rcode 01_hello/hello_world.rs 

A better example would be something like the following.

rcode 01_hello/hello_world2.rs

Several things are going on in this code. First, variables are declared using 
the `let` keyword. Notice that we didn't have to specify a type, such as `str`.
The Rust compiler can infer types, as long as there isn't any ambiguity. 

Second, we specified a return type using `->`. This is fairly straightforward. 
Any valid type can be returned by a function.

Third, there's a `&` in the return type. This is will discussed in the [third 
section](../03_pointers/pointers.md). The [next section]
(../02_variables/variables.md) deals with the basics of Rust variables.
