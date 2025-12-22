fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    // println!("{:?}",arguments);
    if arguments.len() >= 2 {
        let command = std::process::Command::new(&arguments[1]).args(&arguments[2..]).output();
        if command.is_err(){ println!("[ERROR] {}", command.err().unwrap())}

    }
}

