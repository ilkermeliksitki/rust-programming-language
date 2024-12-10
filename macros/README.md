You can use the `cargo expand` command to investigate macros.


```bash
melik@debian:~/rust-programming-language/macros$ cargo expand
    Checking macros v0.1.0 (~/rust-programming-language/macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let x = 10;
    let y = 20;
    let z = x + y;
    {
        ::std::io::_print(format_args!("The sum of {0} and {1} = {2}\n", x, y, z));
    };
    let a = 30;
    let b = 40;
    let c = a + b + z;
    {
        ::std::io::_print(
            format_args!("The sum of {0}, {1} and {2} = {3}\n", a, b, z, c),
        );
    };
}
```
