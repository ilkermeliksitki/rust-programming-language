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
