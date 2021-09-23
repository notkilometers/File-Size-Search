# File-Size-Search
searches files from given root directory and subfolders, returns sorted vector with name and size

## Usage:

```Rust
fn main() {
    let mut v = Vec::<File>::new();
    search_folder("C:\\Users", &mut v);
    sort_vector(&mut v);
    println!("{:?}", v);
}
```
