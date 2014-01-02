Rust Crash Course
============

## Table of Contents
1. [Hello World](markdown/01.md)
2. [Variables and Syntax](markdown/02.md)
3. [Pointers](markdown/03.md)

There's a little preprocessor to make including code easier. It strips the last
3 characters from a file's name and appends `.md`. The original file is left
untouched. These unprocessed files end with `.dm`.

A makefile will eventually be created to make things easier, but, at the moment, everything `.dm` file in `files/` will be processed by running `/.preprocess`. The generated markdown files will be put in `markdown/`.

There's currently a bug in the preprocessor. It's not smart enough to figure
out when file directories have changed. So, if it's being run from the top
directory, the `rcode` embeddings have to be `rcode path/to/file.rs x y` instead
 of just `rcode file.rs x y`.
