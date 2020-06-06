# proc\_use_inline
`proc_use_inline` is a Rust crate offering an alternate syntax for `mod` and `use`
statements. Take the following Rust code:
```rust
mod foo;
use foo::*;

use itertools::*;

#[path = "../other/ext.rs"]
mod ext;
```
This literally translates into:
```rust
proc_macro_inline::proc_macro_inline! {
	#[mod]
	use foo::*;
	
	use itertools::*;
	
	mod("../other/ext.rs");
}
```
