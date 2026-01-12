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

fn wc(file: &str) {
    let lines = read_lines(&file);
    let mut line_count: u64 = 0;
    let mut word_count: u64 = 0;
    let mut byte_count: u64 = 0;

    for line in lines.iter() {
        line_count += 1;
        word_count += line.split_whitespace().count() as u64;
        byte_count += line.as_bytes().len() as u64;
    }

    println!("Line Count {}, Word Count {}, Bytes {}", line_count, word_count, byte_count);
}


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}