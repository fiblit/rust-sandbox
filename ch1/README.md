# Getting Started

## Installing Rust

`rustup update` : updates the Rust tools

`rustc --version` : checks the version/commit/date of Rust tools

`rustup doc` : opens offline documentation for all Rust

## Hello, World!

Functions: `fn main() {}`

Main is *always* called first.

Rust = 4 spaces

println! = macro (!)

Ahead of time compiled

## Hello, Cargo!

Built-in Package manager.

`cargo new <name>` : Generates new project with Cargo.toml, git repo, and src/

`cargo build` : builds exectuable into target/debug/<name>

`cargo run` : builds and runs executable

`cargo check` : will the program compile? No executable.

`cargo build --release` : Basically -O3. Goes to target/release/<name>

