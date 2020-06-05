use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub struct UseBuilder {
    stmts: Vec<String>,
}

impl UseBuilder {
    pub fn new() -> Self {
	Self{stmts: Vec::new()}
    }
    pub fn mod_file(&mut self, file: PathBuf) -> &mut Self {
	let file = match file.canonicalize() {
	    Ok(file) => file,
	    Err(err) => panic!("Could not canonicalize file '{}': {}", file.to_string_lossy(), err),
	};
	match (file.file_stem(), file.extension()) {
	    (Some(mod_name), Some(ext)) if ext == "rs" => {
		self.stmts.push(format!("#[path = \"{}\"]\nmod {};\n",
					file.to_string_lossy(), mod_name.to_string_lossy()));
	    },
	    (Some(_), _) => panic!("Invalid file '{}'. Probable cause: file is not a rust file.",
				   file.to_string_lossy()),
	    (None, _) => panic!("Invalid file '{}'. Probable cause: is not a regular file.",
				file.to_string_lossy()),
	}
	self
    }
    pub fn use_file(&mut self, file: PathBuf, use_stmt: String) -> &mut Self {
	let file = match file.canonicalize() {
	    Ok(file) => file,
	    Err(err) => panic!("Could not canonicalize file '{}': {}", file.to_string_lossy(), err),
	};
	match (file.file_stem(), file.extension()) {
	    (Some(mod_name), Some(ext)) if ext == "rs" => {
		self.stmts.push(format!("#[path = \"{}\"]\nmod {};\nuse {}::{};", file.to_string_lossy(),
					mod_name.to_string_lossy(), mod_name.to_string_lossy(), use_stmt));
	    },
	    (Some(_), _) => panic!("Invalid file '{}'. Probable cause: file is not a rust file.",
				   file.to_string_lossy()),
	    (None, _) => panic!("Invalid file '{}'. Probable cause: is not a regular file.",
				file.to_string_lossy()),
	}
	self
    }
    pub fn use_crate(&mut self, use_stmt: String) -> &mut Self {
	self.stmts.push(format!("use {};", use_stmt));
	self
    }
    pub fn write_to_file(&mut self, path: PathBuf) {
	match File::create(&path) {
	    Ok(mut file) => {
		if let Err(err) = file.write_all(self.stmts.join("\n").as_bytes()) {
		    panic!("Could not write to file '{}': {}", path.to_string_lossy(), err);
		}
	    },
	    Err(err) => panic!("Could not open file '{}' for writing: {}",
			       path.to_string_lossy(), err),
	}
    }
}
