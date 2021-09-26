# File-Size-Search
searches files from given root directory and subfolders, returns sorted vector with name and size

## Usage:
### Windows:
```Rust
fn main() {
    let mut files = Vec::<File>::new();
    search_folder("C:\\Users", &mut files);
    sort_vector(&mut files);
    print_files(&mut files, 10);
}
```
### Linux:
```Rust
fn main() {
    let mut files = Vec::<File>::new();
    search_folder("/home", &mut files);
    sort_vector(&mut files);
    print_files(&mut files, 10);
}
```
