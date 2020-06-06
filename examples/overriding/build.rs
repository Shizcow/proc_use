use proc_use::UseBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    let plugin = false; // Change me!
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut override_watch = vec!["src/reference.rs"];
    let mut usebuilder = UseBuilder::new();
    
    if plugin {
	override_watch.push("src/plugin.rs");
	usebuilder.use_file("src/plugin.rs".into(), "*".into());
    }
    
    overrider_build::watch_files(override_watch);
    usebuilder.use_file("src/reference.rs".into(), "*".into())
	.write_to_file_all(out_path.join("proc_use.rs"));   
}
