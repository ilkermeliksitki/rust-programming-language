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

```rust
rustc hello_world.rs
```

```bash
./hello_world
```

> [!IMPORTANT]
> rust binary files are bigger than the `average` binary file size because of static linking,
> debug information, standard library, etc.


