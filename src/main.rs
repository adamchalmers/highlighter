use std::{io::Read, process::ExitCode};

fn main() -> ExitCode {
    let usage = "usage: highlighter [start offset] [end_offset?]\nReads from stdin. Accepts a start offset (number, required) and end offset (number, optional)";

    // Parse args
    let args: Vec<_> = std::env::args().collect();
    let Some(start_str) = args.get(1) else {
        println!("{usage}");
        return ExitCode::FAILURE;
    };
    let Ok(start) = start_str.parse::<usize>() else {
        println!("Invalid `start`, was not a number");
        println!("{usage}");
        return ExitCode::FAILURE;
    };
    let end_str = args.get(2);
    let end: Option<usize> = match end_str.map(|num| num.parse()) {
        Some(Ok(n)) => Some(n),
        Some(Err(_)) => {
            println!("Invalid `end`, was not a number");
            println!("{usage}");
            return ExitCode::FAILURE;
        }
        None => None,
    };

    // Parse stdin
    let mut buf: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();
    let Ok(string) = String::from_utf8(buf) else {
        println!("Stdin was not a valid string.");
        return ExitCode::FAILURE;
    };

    // Highlight stdin via the given args
    print_highlight(start, end, &string);
    ExitCode::SUCCESS
}

fn print_highlight(start: usize, end: Option<usize>, string: &str) {
    let end = end.unwrap_or(start + 1);
    print!("{}", &string[..start]);
    let red = format!("\x1b[31m{}\x1b[0m", &string[start..end]);
    print!("{red}");
    print!("{}", &string[end..]);
}
