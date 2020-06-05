use proc_use_build::UseBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    //UseBuilder::new().add_file("external/foo.rs").write_to_file(out_path.join("proc_mod.rs"));
}
