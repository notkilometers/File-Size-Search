# FileSearch
searches files from given root directory and subfolders, returns sorted vector with name and size

## Usage:
### Windows:
```Rust
fn main() {
    let mut files = FileSearch::search("C:/);
    FileSearch::sort(&mut files);
    FileSearch::print(&mut files);
}
```
### Linux:
```Rust
fn main() {
    let mut files = FileSearch::search("/home");
    FileSearch::sort(&mut files);
    FileSearch::print(&mut files);
}
```
