use std::fs::read;
use clap::Parser;

/// hdump
/// env!("CARGO_PKG_NAME") by Abhinav Gunwant (github.com/abhinavgunwant)
///
/// Dumps a binary file in the hexadecimal representation.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Name of the file to read
    #[arg(value_name = "file-name")]
    file_name: String
}

fn main() {
    let args = Args::parse();

    match read(args.file_name) {
        Ok(bytes) => {
            let mut bytes_in_line: u8 = 0;
            let mut current_line: Vec<u8> = Vec::with_capacity(16);

            for byte in bytes.iter() {
                print!("{:02X?} ", byte);
                current_line.push(*byte);
                bytes_in_line += 1;

                if bytes_in_line == 16 {
                    print_current_line_ascii(&current_line);
                    current_line.clear();
                    print!("\n");
                    bytes_in_line = 0;
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn print_current_line_ascii(line_bytes: &Vec<u8>) {
    for byte in line_bytes.iter() {
        let c = *byte as char;

        match c {
            '0'..='9'|'a'..='z'|'A'..='Z'|'['|']'|'('|')'|'!'|'@'|'#'|'$'|'%'
                |'^'|'&'|'*'|'-'|'_'|'='|'+'|','|'<'|'.'|'>'|'/'|'?'|'{'|'}'
                |';'|':'|'\''|'"'|'\\'|'|'=> {
                print!("{}", c);
            }
            _ => { print!("."); }
        }
    }
}

