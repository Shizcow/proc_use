use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use glob::glob;

pub struct UseBuilder {
    mod_stmts: Vec<String>,
    use_stmts: Vec<String>,
    unused: bool, // #[allow(unused_imports)] ?
}

impl UseBuilder {
    pub fn new() -> Self {
	Self{mod_stmts: Vec::new(), use_stmts: Vec::new(), unused: true}
    }
    
    pub fn mod_file(&mut self, file: PathBuf) -> &mut Self {
	self.file(file, None);
	self
    }
    pub fn use_file(&mut self, file: PathBuf, use_stmt: String) -> &mut Self {
	self.file(file, Some(use_stmt));
	self
    }
    pub fn use_crate(&mut self, use_stmt: String) -> &mut Self {
	self.use_stmts.push(format!("use {};", use_stmt));
	self
    }
    
    pub fn mod_glob(&mut self, globstring: &str) -> &mut Self {
	for entry in glob(globstring).expect("Failed to read glob pattern") {
	    match entry {
		Ok(path) => self.file(path, None),
		Err(e) => panic!("Could not resolve glob pattern: {:?}", e),
	    }
	}
	self
    }
    pub fn use_glob(&mut self, globstring: &str, use_stmt: String) -> &mut Self {
	for entry in glob(globstring).expect("Failed to read glob pattern") {
	    match entry {
		Ok(path) => self.file(path, Some(use_stmt.clone())),
		Err(e) => panic!("Could not resolve glob pattern: {:?}", e),
	    }
	}
	self
    }

    // #[allow(unused_imports)] on each use statement (does not affect import contents)
    pub fn allow_unused(&mut self) -> &mut Self {
	self.unused = true;
	self
    }
    pub fn warn_unused(&mut self) -> &mut Self {
	self.unused = false;
	self
    }
    
    pub fn write_to_file_use(&mut self, path: PathBuf) -> &mut Self {
	self.write_to_file(path, 
			   if self.unused {
			       self.use_stmts.iter()
				   .map(|s| format!("#[allow(unused_imports)]\n{}", s)).join("\n")
			   } else {
			       self.use_stmts.join("\n")
			   }.as_bytes());
	self
    }
    pub fn write_to_file_mod(&mut self, path: PathBuf) -> &mut Self {
	self.write_to_file(path, self.mod_stmts.join("\n").as_bytes());
	self
    }
    pub fn write_to_file_all(&mut self, path: PathBuf) -> &mut Self {
	self.write_to_file(path,
			   self.mod_stmts.iter().chain(std::iter::once(
			       &if self.unused {
				   self.use_stmts.iter()
				       .map(|s| format!("#[allow(unused_imports)]\n{}", s)).join("\n")
			       } else {
				   self.use_stmts.join("\n")
			       }
			   )).join("\n").as_bytes());
	self
    }
    
    fn file(&mut self, file: PathBuf, use_stmt: Option<String>) {
	let file = match file.canonicalize() {
	    Ok(file) => file,
	    Err(err) => panic!("Could not canonicalize file '{}': {}",
			       file.to_string_lossy(), err),
	};
	match (file.file_stem(), file.extension()) {
	    (Some(mod_name), Some(ext)) if ext == "rs" => {
		self.mod_stmts.push(format!("#[path = \"{}\"]\nmod {};",
					    file.to_string_lossy(),
					    mod_name.to_string_lossy()));
		if let Some(use_stmt) = use_stmt {
		    self.use_stmts.push(format!("use {}::{};",
						mod_name.to_string_lossy(),
						use_stmt));
		}
	    },
	    (Some(_), _) => panic!("Invalid file '{}'. Probable cause: \
				    file is not a rust file.",
				   file.to_string_lossy()),
	    (None, _) => panic!("Invalid file '{}'. Probable cause: is not a regular file.",
				file.to_string_lossy()),
	}
    }
    fn write_to_file(&mut self, path: PathBuf, contents: &[u8]) {
	match File::create(&path) {
	    Ok(mut file) => {
		if let Err(err) = file.write_all(contents) {
		    panic!("Could not write to file '{}': {}", path.to_string_lossy(), err);
		}
	    },
	    Err(err) => panic!("Could not open file '{}' for writing: {}",
			       path.to_string_lossy(), err),
	}
    }
}
