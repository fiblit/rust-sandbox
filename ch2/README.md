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

Rust doesn't have randomness in std, but it does have a random crate (library)

`rand` is a *library* crate. `guessing_game` is a *binary* crate.

### Using a Crate to Get More Funcionality

Semantic versioning is used for crates, and 0.3.14=^0.3.14

Cargo checks `[dependencies]` for any uninstalled Crates. It will download and
install them for you, including secondary dependencies.

`cargo build` works like make where it will only rebuild the changed parts.

### Cargo.lock

The .lock file tracks all downloaded packages.

`cargo update` will only update the minor version v0.3.14 -> 0.3.15

### Genreating a Random Number

`extern crate <crate>` use external dependency. (adds a `use::<crate>`)

`thread_rng()` is a thread_local RNG seeded by the OS.

`gen_range(a, b)` is uniformly random over [a, b)

`cargo doc --open` will open all dependency's documentation

## Comparing the Guess to the Secret

`cmp` can be compared on anything.

match statements match patterns in arms.

Rust is strongly statically typed.

Annotate types when it can't be inferred `let var: type`

## Allowing Multiple Guesses with Looping

`loop` = infinite loop

### Quitting after a Guess

arms can have braces `=> {}`. This lets them have multiple statements

### Handling Invalid Input

You can match `Result` to be more intelligent. Using this instead of expect is
like the difference between assert and error handling.
