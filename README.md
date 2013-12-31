RustTutorial
============

There's a little preprocessor to make including code easier. It strips the last
3 characters from a file's name and appends `.md`. The original file is left
untouched. These unprocessed files end with `.dm`.

A makefile will eventually be created to make things easier, but at the moment
a file is processed by compliling `preprocessor.cpp` and running
`./a.out <file_to_be_processed>`.
