use std::{fs, fmt};

// File struct containing the path and the size of file
struct File {
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

// searches folders until reaches leaf, adds all files to vector
fn search_folder(directory: &str, files: &mut Vec<File>) {
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
            search_folder(path.to_str().unwrap(), files);
        }
    }
}

// sort vector by comparing file size, reverse so output ends with largest files
fn sort_vector(files : &mut Vec<File>) {
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files.reverse();
}

// prints n files from vector
fn print_files(files: &mut Vec<File>, n: i32) {
    for _i in 1..=n {
        println!("{:?}", files);
    }
}

fn main() {
    let mut files = Vec::<File>::new();
    search_folder("/home/null", &mut files);
    sort_vector(&mut files);
    print_files(&mut files, 10);
}
