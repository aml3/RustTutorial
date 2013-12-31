RustTutorial
============

There's a little preprocessor to make including code easier. It strips the last
3 characters from a file's name and appends `.md`. The original file is left
untouched. These unprocessed files end with `.dm`.

A makefile will eventually be created to make things easier, but at the moment
a file is processed by compliling `preprocessor.cpp` and running
`./a.out <file_to_be_processed>`.

There's currently a bug in the preprocessor. It's not smart enough to figure
out when file directories have changed. So, if it's being run from the top
directory, the `rcode` embeddings have to be `rcode path/to/file.rs x y` instead
 of just `rcode file.rs x y`.
