# Getting Started

## Hello World

```rust
fn main() {
    println!("Hello, world!");
}
```

> [!NOTE]
> `println!` is a macro, not a function.
>
> Macros don't always follow the same rules as functions. (Chapter 19 for more)

To run the program, use the `rustc` command to compile the program and then run the resulting executable:

```bash
rustc hello_world.rs
```

```bash
./hello_world
```

> [!IMPORTANT]
> rust binary files are bigger than the `average` binary file size because of static linking,
> debug information, standard library, etc.

## Cargo

Cargo is Rust's build system and package manager. Most Rustaceans use Cargo to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

To create a new project, run:

```bash
cargo new hello_cargo
```
Here is the output of the command:

```html
├── hello_cargo
│   ├── Cargo.toml
│   └── src
│       └── main.rs
```

`Cargo.toml` is a configuration file for your project. It contains all the metadata that Cargo needs to compile your project.

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

> Cargo expects your source files to live inside the src directory.

What is `cargo init`?

`cargo init` is a command that creates a new Cargo project in a directory. It is similar to `cargo new`, but it creates the project in the current directory instead of creating a new directory for the project.

### Building and Running a Cargo Project

what is `cargo build`?

`cargo build` is a command that compiles your Rust project. It creates an executable file in the target/debug directory.

Here is the file structure after running `cargo build`:

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── .rustc_info.json
    ├── CACHEDIR.TAG
    └── debug
        ├── .cargo-lock
        ├── .fingerprint
        ├── build
        ├── deps
        ├── examples
        ├── hello_cargo         <== Executable file
        ├── hello_cargo.d
        └── incremental
```
