//! # proc_use
//!
//! proc_use is a library for semi-dynamically importing creates/modules.
//!
//! # About
//! The proc_use library leverages the builder patterns. For more information about the builder pattern find it at [Rust Builder Pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
//!
//! See the proc_use [website](https://docs.rs/proc_use) for additional documentation and usage examples.
//!
//! # Quick Example
//! project_root/src/main.rs
//! ```
//! include!(concat!(env!("OUT_DIR"), "/proc_use.rs"));
//! fn main() {
//!     foo();
//! }
//! ```
//!
//! project_roo/external/foo.rs
//! ```
//! pub fn foo() {
//!     println!("Hello from foo!");
//! }
//! ```
////////////////////////////////////////////////////////////////////////////////
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use glob::glob;

/// The struct to represent the builder for proc_use.
pub struct UseBuilder {
    /// The mod statements to be generated.
    mod_stmts: Vec<String>,
    /// The use statements to be generated.
    use_stmts: Vec<String>,
    /// Whether the import is used or not.
    unused: bool, // #[allow(unused_imports)] ?
}

impl UseBuilder {
    /// Returns a UserBuilder with no mod or use statemetns and unused defaulted to true.
    pub fn new() -> Self {
	Self{mod_stmts: Vec::new(), use_stmts: Vec::new(), unused: true}
    }

    /// Adds a file to mod to the builder.
    ///
    /// # Arguments
    ///
    /// * `file` - A PathBuf to the file that needs to be modded.
    pub fn mod_file(&mut self, file: PathBuf) -> &mut Self {
	self.file(file, None);
	self
    }

    /// Adds a file to use to the builder, and use pattern.
    ///
    /// # Arguments
    ///
    /// * `file` - A PathBuf to the file that needs to be modded.
    /// * `use_stmt` - A String that holds the use pattern for the file.
    pub fn use_file(&mut self, file: PathBuf, use_stmt: String) -> &mut Self {
	self.file(file, Some(use_stmt));
	self
    }

    /// Adds a crate to use to the builder with a use pattern.
    ///
    /// # Arguments
    ///
    /// * `use_stmt` - A String that holds the use pattern for the crate.
    pub fn use_crate(&mut self, use_stmt: String) -> &mut Self {
	self.use_stmts.push(format!("use {};", use_stmt));
	self
    }

    /// Glob a file path for files to mod.
    ///
    /// # Arguments
    ///
    /// * `globstring` - The glob pattern string to match.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = UseBuilder::new()
    ///         .mod_glob("src/util/*.rs");
    /// ```
    pub fn mod_glob(&mut self, globstring: &str) -> &mut Self {
	for entry in glob(globstring).expect("Failed to read glob pattern") {
	    match entry {
		Ok(path) => self.file(path, None),
		Err(e) => panic!("Could not resolve glob pattern: {:?}", e),
	    }
	}
	self
    }

    /// Glob a file path for files to use.
    ///
    /// # Arguments
    ///
    /// * `globstring` - The glob pattern string to match.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = UseBuilder::new()
    ///         .use_glob("src/util/*.rs", "*".into());
    /// ```
    pub fn use_glob(&mut self, globstring: &str, use_stmt: String) -> &mut Self {
	for entry in glob(globstring).expect("Failed to read glob pattern") {
	    match entry {
		Ok(path) => self.file(path, Some(use_stmt.clone())),
		Err(e) => panic!("Could not resolve glob pattern: {:?}", e),
	    }
	}
	self
    }

    /// Disables the unused import error for each import.
    pub fn allow_unused(&mut self) -> &mut Self {
	self.unused = true;
	self
    }

    /// Enables the unused import error for each import.
    pub fn warn_unused(&mut self) -> &mut Self {
	self.unused = false;
	self
    }

    /// Writes the use statements to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - A PathBuf to the file that is to be written.
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

    /// Writes the mod statements to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - A PathBuf to the file that is to be written.
    pub fn write_to_file_mod(&mut self, path: PathBuf) -> &mut Self {
	self.write_to_file(path, self.mod_stmts.join("\n").as_bytes());
	self
    }

    /// Writes the mod and use statements to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - A PathBuf to the file that is to be written.
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

    /// Helper function to canonicalizes a file, and adds use or mod statements.
    ///
    /// # Arguments
    ///
    /// * `file` - A PathBuf to the file that needs to be validated and moded/used.
    /// * `use_stmt` - A optional String that holds the use pattern for the file.
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

    /// Helper function to write to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - A PathBuf to the file that needs to be modded.
    /// * `contents` - The contents to be written to a file.
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
