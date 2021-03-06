use proc_use::UseBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    UseBuilder::new()
		.use_glob("src/util/*.rs", "*".into())
		.write_to_file_all(out_path.join("proc_use.rs"));
}
