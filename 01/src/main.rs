use std::env;

fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();

    if command_line_arguments.len() < 2 {
        eprintln!("Usage: {} <file>", command_line_arguments[0]);
        std::process::exit(1);
    }

    let file_path: &str = &command_line_arguments[1];
    wc(file_path);
}

fn wc(file_path: &str) {
    let file_contents: String = std::fs::read_to_string(file_path).unwrap();

    let byte_count: u64 = file_contents.as_bytes().len() as u64;
    let line_count: u64 = file_contents.as_bytes().iter().filter(|byte| **byte == b'\n').count() as u64;
    let word_count: u64 = file_contents.split_whitespace().count() as u64;

    println!("Line Count {}, Word Count {}, Bytes {}", line_count, word_count, byte_count);
}
