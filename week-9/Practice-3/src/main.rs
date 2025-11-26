use std::fs;

fn main() {
    fs::remove_file("chigozirim_data.txt").expect("could not remove file");
    println!("file is removed");
}