use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PATH: &str = "test.csv";

struct Event {
    event: String,
    time: String,
    date: String,
    fire_type: String,
    is_in: bool,
    address: Vec<i8>,
    message: String,
}

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

fn create_date(dates: Vec<i32>) -> String {
    let mut new_date = String::new();

    new_date.push_str(&dates[1].to_string());
    new_date.push('/');
    new_date.push_str(&dates[0].to_string());
    new_date.push('/');
    new_date.push_str(&dates[2].to_string());

    println!("Date: {}", new_date);

    new_date
}

fn create_time(time: Vec<i8>) -> String {
    let mut new_time = String::new();
    for (i, timing) in time.into_iter().enumerate() {
        new_time.push_str(&timing.to_string());
        match i {
            0 | 1 => new_time.push(':'),
            _ => break,
        }
    }
    println!("Time: {}", new_time);

    new_time
}

fn create_address(address: Vec<i16>) -> String {
    let mut joined_address = String::new();
    for (i, addr) in address.into_iter().enumerate() {
        match i {
            0 => joined_address.push_str("Node: "),
            1 => joined_address.push_str(" Loop: "),
            _ => joined_address.push_str(" Address: "),
        }
        joined_address.push_str(&addr.to_string());
    }
    println!("Module = {}", joined_address);

    joined_address
}

fn create_event(line: &str) {
    let mut event = String::new();
    let mut date: Vec<i32> = Vec::new();
    let mut time: Vec<i8> = Vec::new();
    let mut fire_type = String::new();
    let mut is_in = false;
    let mut address: Vec<i16> = Vec::new();
    let mut message = String::new();

    for (i, line) in line.split(",").enumerate() {
        match i {
            0 => event.push_str(line),
            1..=3 => date.push(line.parse().unwrap()),
            4..=6 => time.push(line.parse().unwrap()),
            7 => fire_type.push_str(line),
            8 => {
                if line.contains(" IN") {
                    is_in = !is_in;
                }
            }
            9..=10 => continue,
            11..=13 => address.push(line.parse().unwrap()),
            _ => message.push_str(line),
        }
    }

    create_date(date);
    create_time(time);
    create_address(address);
    println!("Message = {}", message);
    println!();
}

fn create_command_event(line: &str) {
    let mut event = String::new();
    let mut date: Vec<i32> = Vec::new();
    let mut time: Vec<i8> = Vec::new();
    let mut status = String::new();
    let mut command_id: i32 = 0;
    let mut operator = String::new();
    let mut operator_priv: &str = "";
    let mut message = String::new();

    for (i, col) in line.split(',').enumerate() {
        match i {
            0 => event.push_str(col),
            1..=3 => date.push(col.parse().unwrap()),
            4..=6 => time.push(col.parse().unwrap()),
            9 => status.push_str(col),
            10 => command_id = col.parse().unwrap(),
            11 => operator.push_str(col),
            12 => operator_priv = col,
            7..=8 => continue,
            _ => message.push_str(col),
        }
    }

    create_date(date);
    create_time(time);
    println!("Command ID: {}", command_id);
    println!("Operator: {}", operator);
    println!("Operator Privilege: {}", operator_priv);
    println!("Message = {}", message);
    println!();
}

fn main() {
    let file_lines = get_file_lines();
    for line in file_lines {
        let event = line.split(",").next().unwrap();
        if event.contains("COMMAND") {
            create_command_event(&line);
        } else {
            create_event(&line);
        }
    }
}
