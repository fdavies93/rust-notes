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

#### Variables and Mutability

`const` can be used in the global scope but `let` can only be used in a function.

Can reuse a variable name if you shadow it. This also lets you change the type, which you can't do with a mutable.

```rust
// this succeeds
let spaces = "   ";
let spaces = spaces.len();
```

```rust
// this fails
let mut spaces = "   ";
spaces = spaces.len();
```

#### Data Types

In this example, we need to tell the compiler the type of guess explicitly.
`let guess: u32 = "42".parse().expect("Not a number!");`

**In C** char = 1 byte = 8 bits = 256 possibilities = 2^8

**In Rust** char = 4 bytes = 32 bits = 4294967296 possibilities = 2^32

##### Tuples

Rust supports tuples.

```rs
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Tuples can store multiple values with different types.

##### Arrays

Arrays have a fixed length once created. Vectors from the standard library take the place of dynamic.

Arrays are useful when you want to put data on the stack rather than the heap.

```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5];
```

#### Functions

We use snake case (my_cool_function) in Rust.

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}
```

> Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

You need to put down the type of a function in Rust:

```rs
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

> Statements are instructions that perform some action and do not return a value.
> Expressions evaluate to a resultant value. Let’s look at some examples.

**Expressions must not have a semicolon; statements must have one.**

```rs
fn main() {
    let y = 6;
}
```

This is not good because declaring something isn't an expression. So the following Python is not good Rust.

```py
    x = y = z = 0
```

```rs
fn main() {
    let x = (let y = 6);
}
```

You can use curly brackets to create an expression from inner scope. The following is valid:

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 

Functions must indicate their return type:

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

#### Comments

Rust uses C-style comments. (//)

#### Control Flow

If statements don't require parentheses but are otherwise standard C-style.

```rs
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

We can use if {} else {} as a ternary operator due to it being an expression.

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

The `loop` keyword is essentially `while(true)`:

```rs
fn main() {
    loop {
        println!("again!");
        // can use break; here
        // can use continue
    }
}
```

Loops can return values on break because they are expressions:

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

You can label loops in Rust and use that to control which scope break statements are targeted at.

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

While loops use regular syntax:

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

For loops use for-in syntax. The standard way to loop x times is to use a `Range` construct (as in Python).

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

### Ownership and the Borrow Checker

#### Stack and Heap

By default, Rust puts **everything** on the stack. This means:
* You can't modify the length of normal arrays.
* You can't modify string literals.

#### Ownership Rules

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

#### Application to Strings

```rs
let myStr = "hello";
myStr += "that";
// this throws an error
```

```rs
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
// this does not
```

When a variable falls out of scope, the program calls drop().

In this example, s1 will cease to point to the piece of memory after s2 is allocated and will therefore throw an error. While similar to a shallow copy, because we invalidate the original pointer, it's called a *move*.

```rs
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

We can use .clone() on many common data types to copy and allocate a new pointer for some block of memory (i.e. a deep copy).

```rs
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

Integers (and some other types) always copy their memory values on allocation because they are simple and fast to copy.

We can allocate a `Copy` trait to custom data types to make this happen, but we can't allocate it to data types with `Drop` trait.

The following data types have the `Copy` trait:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating-point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

