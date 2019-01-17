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
