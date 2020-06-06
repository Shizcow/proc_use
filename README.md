# proc_use
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
And in build.rs:
```rust
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
To see an example using this directory structure, see [globbing](examples/globbing).

