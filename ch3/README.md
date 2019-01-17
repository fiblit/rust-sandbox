# Common Programming Concepts

## Introduction

### Keywords

There is a list in Appendix A

### Identifiers (Raw, too)

`-` cannot be an identifer by itself, otherwise the usual rules.

Raw identifiers, however, can be specified to overwrite keywords using `r#`, for
example: `r#match();`. However, you should really only use this for wrapping
other languages' libraries.

## Variables and Mutability

By default immutable, such that you cannot change a value once bound.

Use `mut` to make variables mutable.

### Variables vs. Constants

Constants, like immutable variables, cannot change their value once bound. You
cannot change this with `mut`, however, they are *always* immutable.

Declare with `const` and an explicit type annotation.

Must be defined as a *constant expression*.

(For long numeric literals, you may use `_` as a delimiter.)

### Shadows

New variables can be *declared* (using `let`) with the same name as prior ones.

Shadowing is like a transformation, however it is still not assignable (as `mut`
would allow). For example, the type can be changed. `mut` could not transform
across types like this because it only allows ("strongly") safe assignments.

## Data Types

Every value has a type. Furthermore, *static* typing implies that the type of
all variables is inferred at compile time. If multiple types are possible, you
must annotate, such as in `.parse()`.

### Scalars (ints, floats, bools, chars)

Singular values, such as integers, floats, bools, and characters.

Integers may be (u)nsigned or s(i)gned + 8, 16, 32, 64, 128, or arch size bits.

You may annotate any literal with the integer types as well. Other bases can
use typical prefixes: hex, `0x`; octal, `0o`; binary, `0b`; byte `b'A'`.

In debug mode overflows cause panics. std contains an explicitly wrapping type.

Floats only have `f32` and `f64`. The default is `f64`.

You can use all of the typical arithmetic operations on similar types: `+`, `-`,
`*`, `/`, `%`.

bools are just `true` and `false`.

chars are specified with `'`. A single Unicode value.

### Compounds (tuples, arrays)

Tuples group a variety of types into one, they do not change in size. The type
of tuples can be inferred, as well as inferred from patterened destructuring! 

Arrays are for homogenous, fixed size stack allocation. The vector type can
support the dynamic sizing. Prefer vector if you're unsure as to which.

Array type annotations are weird: `[content; length]`

Arrays can be indexed as expected, in debug though out of bounds errors are
detected at *runtime*.

## Functions

snake_case is the conventional style for function names.

Rust does not care where functions are defined.

As usual, functions can have parameters, however, they *must* have type
annotations.

### Statements vs Expressions

Functions contain a bunch of statements, maybe with an ending expression. (Rust
is expression-based, or whatever that means.)

Statements do not return a value, while expressions do. Therefore, you cannot
assign a statement to a let *statement*.

Functions, macros, and blocks are all expressions. In fact, the final
item of the block can be an expression by not having a semicolon.

### Return Values

You specifiy the type after a function with `-> Type` which is the same as the
final expression's type. Otherwise, return values seem to work as expected.

You cannot make the final expression a statement if the function is declared
with a return type.

## Comments

The only kind of comment appears to be the C++ style `//` comment. No debugging
block comments? It's preferrable to place comments above lines.
