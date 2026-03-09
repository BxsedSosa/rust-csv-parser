use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PATH: &str = "test.csv";

fn clear_console() {
    print!("\x1B[2J")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file_lines() -> Vec<String> {
    let mut file_lines: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(PATH) {
        for line in lines.map_while(Result::ok) {
            file_lines.push(line);
        }
    }

    file_lines
}

fn display_lines(lines: Vec<String>) {
    clear_console();
    for line in lines {
        println!("{}", line);
    }
}
fn main() {
    display_lines(get_file_lines());
}
