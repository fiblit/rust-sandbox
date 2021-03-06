# rust-sandbox

Sandbox for experiments in Rust (and associated tools, like Crate). The repo is
mainly used for recording my progress in learning the language, at least for
right now.

## Lesson Sumarries

### Foreword

* It is about *empowerment*.
* Working with Rust allows you to build skills that transfer from one domain
  to another.
* It’s a friendly and approachable text intended to help you level up not
  just your knowledge of Rust, but also your reach and confidence as a
  programmer in general.

### Introduction

* Ideal for:
  * Teams
  * Students
  * Companies
  * Open Source Developers
* Safe code = Fast code
* Later chapters build on concepts in earlier chapters,
* we’ll provide many examples of code that doesn’t compile along with
  the error message.

### Ch 1: Getting Started

* Installing Rust
* Hello, world!
* Using `cargo`

### Ch 2: Programming a Guessing Game

* Setting up a New Project
* Processing a Guess
  * Storing Values with Variables
  * `Result` = optional failure
  * Printing with `println!`
* Generating a secret number
  * Using a Crate
  * Cargo.lock
  * Generating a random number
* Comparing the Guess to the Secret Number
* Looping
* Proper Error Handling w/ Match

### Ch 3: Common Programming Concepts

* Introduction
  * Keywords (Appendix A)
  * Identifiers (Raw, too)
* Variables and Mutability
  * Variables vs. Constants
  * Shadows
* Data Types
  * Scalars (ints, floats, bools, chars)
  * Compounds (tuples, arrays)
* Functions
  * Parameters
  * Statements vs Expressions
  * Return Values
* Comments
* Control Flow *expressions*
* Loops

### Ch 4: Understanding Ownership

* What is it?
  * The Stack vs. The Heap
  * Rules
    * Scope
    * `String` type
    * Memory / Allocation
      * Move
      * Clone
      * Copy
    * Ownership with functions
      * Return Values
* References and Borrowing
  * Mutable References
  * Dangling References
* Slices
  * String literals are slices
  * String slice parameters
  * Other Slice types
