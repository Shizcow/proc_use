# proc_use &emsp; [![Latest Version]][crates.io] [![API]][documentation] [![proc_use: rustc 1.43.1+]][Rust 1.43.1]

[Latest Version]: https://img.shields.io/crates/v/proc_use.svg
[crates.io]: https://crates.io/crates/proc_use
[API]: https://docs.rs/proc_use/badge.svg
[documentation]: https://docs.rs/proc_use
[proc_use: rustc 1.43.1+]: https://img.shields.io/badge/proc_use_1.43.1+-lightgray.svg
[Rust 1.43.1]: https://blog.rust-lang.org/2020/05/07/Rust.1.43.1.html

`proc_use` is a Rust crate to help semi-dynamically import crates and modules.
All logic is ran at compile time, mostly contained in `build.rs`.

## Use case
In what scenario is this crate useful? Say you have the following directory structure:  
```
project
├─Cargo.toml
├─build.rs
└─src
  ├─main.rs
  ├─core
  │ └─core.rs
  └─util
    ├─foo.rs
    ├─bar.rs
    └─baz.rs
```
You find yourself often adding new files to `util` and including them in `core.rs`.
The way to do this under vanilla Rust is like this:
```rust
// main.rs
#[path = "util/foo.rs"]
mod foo;
#[path = "util/bar.rs"]
mod bar;
#[path = "util/baz.rs"]
mod baz;
```
```rust
// core.rs
use foo::*;
use bar::*;
use baz::*;
```
Of course there are slightly cleaner ways to do this to avoid `#[path]` but that's more
work. And you need to edit multiple files every time you add a new util.
Annoying! This is where `proc_use` comes in. The above can be replaced with:
```rust
// core.rs
include!(concat!(env!("OUT_DIR"), "/proc_use.rs"));
```
```rust
// build.rs
use proc_use::UseBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    UseBuilder::new()
		.use_glob("src/util/*.rs", "*".into())
		.write_to_file_all(out_path.join("proc_use.rs"));
}
```
That's more code up front, but now the mod and use process is automatic.
Add as many Rust files to `util` as you desire; the `use_glob` method will
pick up and import all of them.  
To see an example using this directory structure, see
[globbing](https://github.com/Shizcow/proc_use/tree/master/examples/globbing).

