use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "hello.txt";

fn main() {
    let data = "Hello World";
    fs::write(FILENAME, data)
        .expect("Unable to write file.");
    println!("Write complete.");

    let new_data = fs::read_to_string(FILENAME)
        .expect("Unable to read file.");
    println!("Read: {}", new_data);

    let mut file_ref = OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .expect("Unable to open file.");

    file_ref.write_all("\nHello again.".as_bytes())
        .expect("Unable to append file");
    println!("Append complete.");

    let new_data = fs::read_to_string(FILENAME)
        .expect("Unable to read file.");
    println!("Read: {}", new_data);
}

// Read File

// Open File

// Append to File