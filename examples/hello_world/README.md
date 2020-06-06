# hello_world
This example shows the basics of `proc_use`. There are two important parts:
1. `build.rs`
   This generates the `mod` and `use` statements.
2. src/main.rs
   This file contains one interesting line of code (`include!`) which uses the
   statements generated in `build.rs`.
Additionally, `external/foo.rs` is where the import points.
It shows how arbitrary paths can be easily used with `proc_macro`
