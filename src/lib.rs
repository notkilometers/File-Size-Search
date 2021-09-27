#![allow(non_snake_case)]
#![crate_type = "lib"]
#![crate_name ="FileSearch"]

use std::{fs, fmt};

// File struct containing the path and the size of file
pub struct File {
    path: String,
    size: u64
}

// impl for printing File
impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // prints path & size of file
            write!(f, "({}, {})\n", self.path, self.size)
    }
}
    
// performs traversal of branches & files
pub fn search (directory: &str) -> Vec<File> {
    // create vector to add files to
    let mut files = Vec::<File>::new();
    // gets folder, reads
    let folder = fs::read_dir(directory).expect("Invalid Dir");
    // for each entry (file, directory) in the folder 
    for entry in folder {
        // gets path
        let path = entry.unwrap().path();
        // gets metadata for entry
        let metadata = fs::metadata(&*path).unwrap();
        // if entry is a file
        if metadata.is_file() {
            // push file onto vector
            files.push(File { 
                path: path.to_str().unwrap().to_string(),
                size: metadata.len() });
        } 
        // if is directory, call search on subfolder
        else {
            files.append(&mut search(path.to_str().unwrap()));
        }
    }

    files
}

// sorts vector s/t largest files are at the end
pub fn sort(files: &mut Vec<File>) {
    files.sort_by(|a, b| a.size.cmp(&b.size));
}

// prints files from vector
pub fn print(files: &mut Vec<File>) {
    for file in files.iter() {
        println!("{:?}", file);
    }
}
