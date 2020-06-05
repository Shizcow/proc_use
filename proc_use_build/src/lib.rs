use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use quote::quote;

pub struct UseBuilder {
    files: Vec<PathBuf>,
}

impl UseBuilder {
    pub fn new() -> Self {
	Self{files: Vec::new()}
    }
    pub fn add_file(&mut self, file: PathBuf) -> &mut Self { // TODO: template String
	self.files.push(file);
	self
    }
    pub fn add_files(&mut self, mut files: Vec<PathBuf>) -> &mut Self {
	self.files.append(&mut files);
	self
    }
    pub fn write_to_file(&mut self, path: PathBuf) {
	let file_strings: Vec<String> = self.files.iter().map(|s| match s.canonicalize() {
	    Ok(absolute) => absolute.to_string_lossy().into_owned(),
	    Err(err) => panic!("Unable to canonicalize path for file '{}': {}", s.to_string_lossy(), err),
	}).collect();
	match File::create(&path) {
	    Ok(mut file) => {
		if let Err(err) = file.write_all(quote!{
		    proc_use::proc_use! {
			#(mod(#file_strings);)*
		    }
		}.to_string().as_bytes()) {
		    panic!("Could not write to file '{}': {}", path.to_string_lossy(), err);
		}
	    },
	    Err(err) => panic!("Could not open file '{}' for writing: {}", path.to_string_lossy(), err),
	}
    }
}
