fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() >= 2 {
        let command = std::process::Command::new(&arguments[1]).args(&arguments[2..])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output().unwrap_or_else(|e| {
            eprintln!("[ERROR]: {}", e);
            std::process::exit(1);
        });
        if !command.status.success() { 
            match command.status.code() {
                Some(code) =>{
                        let stderr =String::from_utf8_lossy(&command.stderr);
                        if !stderr.is_empty() {
                            eprintln!("[ERROR]: {stderr}");
                        } else {
                            eprintln!("[ERROR]: Exited with status code: {code}");
                        }
               },
                None => eprintln!("[ERROR]: Process terminated by signal")
            };
        }
    } else {
        eprintln!("USAGE:\nquiet <command> [args]");
        std::process::exit(1);
    }
}