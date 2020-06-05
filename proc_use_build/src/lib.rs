use std::path::PathBuf;

pub struct UseBuilder {
    files: Vec<String>,
}

impl UseBuilder {
    pub fn new() -> Self {
	Self{files: Vec::new()}
    }
    pub fn add_file(&mut self, file: &str) -> &mut Self { // TODO: template String
	self.files.push(file.to_string());
	self
    }
    pub fn add_files(&mut self, files: Vec<&str>) -> &mut Self {
	self.files.append(&mut files.into_iter().map(|f| f.to_string()).collect());
	self
    }
    pub fn write_to_file(&self, path: PathBuf) {
	panic!("{:?}", path);
    }
}
