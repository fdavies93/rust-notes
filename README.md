# Rust Notes

## From Book

Super-condensed version of the Rust standard book.

### 1.2
`println!()` is not a function but a macro.
Functions use regular i.e. C-like syntax.

### 1.3 
`cargo new NAME` to create new project\
`cargo new --help` to see options\
`cargo build` to build project\
`./target/debug/NAME` is default build location\
`cargo run` will build & run\
`cargo check` confirms code is good w/o compiling\
`cargo build --release` is slower build, but more optimised\

### 2: Programming a Guessing Game (./guessing_game)
`use std::io` for importing default io (duh)\
`let mut NAME: TYPE` for declaring mutable variables with type\
`&mut guess` passes guess by reference\
`read_line()` returns a Result, which is an enum\
`Result` type has the `expect()` method, which crashes with message if it returns an `Err` type.\
`[dependencies]` in toml uses semantic versioning; it pulls from repo\
`cargo update` to update to new versions and ignore lock file\
`thread_rng` is a threadbound RNG seeded by OS\
`1..=100` is an inclusive range expression from `start..=end`\
`guess.cmp` is a comparison operator; it uses *arms* to compare patterns and values\

If you declare a variable over another type with same name, or make it mutable / immutable, you can *shadow* the original.

Strings have `trim()` and `parse()` methods. User input has a terminal `\n`.

`loop` creates an infinite loop (cool)\
`break` statements work as expected

`match` statements work on the result of a function and let you create arms with different outcomes

### 3: Common Programming Concepts