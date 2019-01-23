# Ownership

## What is it?

This is Rust's *central* feature. Rust uses a deterministic memory management
policy where memory is determined at compile time.

### The Stack vs. The Heap

The main reasoning for ownership is in managing the heap, otherwise the ideas of
using the stack vs the heap are the same.

### Rules

* Every variable has an owner.
* There can only be one owner.
* Values are dropped when the owner goes out of scope.

#### Scope

As expected, scope lasts from { to }. Before the left brace variables don't
exist, and they stop existing at the right-brace.

#### `String` type

All previously discussed types were stack-allocated.

use String::from to convert literals into a mutable String:

```rust
let mut s = String::from("hello");
```

#### Memory / Allocation

String is allocated at runtime on the heap. When it leaves scope its ::drop
function is automatically called. (Kinda like ~dtors?)

##### Move

let s1 = String, then let s2 = s1 does not copy the heap data, only the shallow
structure of the String (its pointer, length, and capacity).

HOWEVER, it also invalidates s1, thus moving the data from s1 to s2.

Rust never does automatic "deep" copies.

##### Clone

To do a deep copy on `String`, use `::clone`. This won't invalidate s1.

##### Copy

So simple things seem to not be invalidated??? Or, rather, only things with the
`Copy` trait are not invalidated. This is placed on types which are so simple
that it only makes sense to have a "shallow" copy. This heuristic is enforced
by only allowing `Copy` on types whose members do not `drop` by implementing
the `Drop` trait. (Appendix C and chapter 10 have more on this.)

Check documentation of a type to see if it's `Copy`able, but...:

* all scalars
* tuples if they contain `Copy`.

### Ownership with Functions

Passing an argument to a function will move or copy, similar to assignment (by
moving this would invalidate items passed in).

#### Return values

Everything works the same: assigning a value to another variable moves it. When
a heap-bearing variable goes out of scope, it calls `drop` *unless* its
ownership has been moved to another variable.

Multiple return values are possible through tuples!

You can take and return ownership within a function, but that's tedious (thus
references!)

## Referenecs and Borrowing

* At any time, you can have *either* one `&mut` or infinite `&`s.
* References must always be in-scope/valid

Avoid the tedious take-and-return of borrowing by using references (&T).

Referencecs do NOT take ownership.

Having reference parameters is *"borrowing"*. You cannot modify borrowed items.

### Mutable References

`mut` references may modify, though! .... except only one-at-a-time in the same
simultaneous scope. This prevents data races, which happen in the perfect storm:

* Two+ pointers access data simultaneously
* One+ pointers is writing to it
* No synchronization mechanism (like mutexes or semaphores)

Thus, you cannot mutably reference something which is referenced. (They don't
expect their value to change!)

### Dangling Refereneces

Guarenteed not to occur. Trying to return a dangling reference complains about
nothing to borrow from! (Lifetimes do something else for it in Chapter 10.)

Instead of returning dangling references, return the value directly (by moving/
copying)!

## Slices

(if you don't want ownership, take a reference.)

Slices do not have ownership. They let you reference a contiguous set of items
instead of the whole collection.

`.as_bytes` on a String turns it into bytes.

`.enumerate()` turns an iterator into (index, &item) pairs. (See Chapter 13)

syntax of slicing: `&s[i..j]`, take a range of s from [i to j) as a reference.

To include the jth element, use `..=`.

Slices store a pointer to their beginning and their length.

You may elide `0` from the start of a slicing range: `[..5]`, for example.

You may elide `s.len()` from the end of a slicing range: `[5..]`, for example.

In fact, you may elide both (for some reason?) to slice the whole string: `[..]`

You cannot slice in the middle of a UTF-8 character. (See chapter 8 for more.)

String slicing type: `&str`

Slices make a dangling reference impossible here, by mutating the string you
cannot also immutabely borrow for a slice.

### String Literals are slices

Literals are immutable because `&str` is immutable.

They slice a point in the binary.

### String Slice Parameters

`&str` may take literals and `&String`s (if turned into a slice..);

### Other Slice types

While `&str` is `String` specific, the general typing is this: `&[T]`. This
refers to part of an array.
