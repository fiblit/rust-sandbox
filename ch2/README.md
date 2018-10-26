# Programming a Guessing Game

Hands-on overview

## Setting Up a New Project

`cargo new <project>` : creates Cargo.toml and subdirs, including a Hello, World! Program

`cargo run` : compiiles and runs.

## Processing a Guess

standard library = std

The prelude is brought into every program.
Types not in the prelude are not brought in.

main is entry point

Declare new function with no parameters: `fn <function>() {`

`println!` is a macro!

### Storing Values with Variables

Let statement binds an immutable foo to bar: `let foo = bar;`

Adding `mut` makes the variable mutable: `let mut...`

C-style `//` comments.

`String` is a UTF-8 text.

`::f` means `f` is an associated/type-attached function (static method).

Not using a type (`std::io`) doesn't mean it's inaccessible, just harder: `std::io::...`.

`read_line` reads a line from some input into an parameter string, and returns an `io::Result`

`&` are references, `&mut` is a mutable reference.

### Handling Potential Failure with the `Result` Type

Break up methods across lines, easier to read!

`io::Result` from `read_line` is an enum(eration). Enums vary over variants

`io::Result` = `Ok` or `Err`.

Rust will warn you if you don't `.expect` a `Result`.

### Printing Values with `println!` Placeholders

Curly brackets `{}` are like % formatters in printf.


## Generating a Secret number
