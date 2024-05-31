use std::fs::File;
use std::io::{self, Read, Write};
use std::env;

fn brainfuck_interpreter(code: &str, input_data: &str) -> String {
    let mut tape = vec![0u8; 30000];
    let mut pointer = 0;
    let mut input_pointer = 0;
    let mut code_pointer = 0;
    let mut output = String::new();
    let code_bytes = code.as_bytes();
    let input_bytes = input_data.as_bytes();

    while code_pointer < code_bytes.len() {
        match code_bytes[code_pointer] {
            b'>' => pointer += 1,
            b'<' => pointer -= 1,
            b'+' => tape[pointer] = tape[pointer].wrapping_add(1),
            b'-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            b'.' => output.push(tape[pointer] as char),
            b',' => {
                if input_pointer < input_bytes.len() {
                    tape[pointer] = input_bytes[input_pointer];
                    input_pointer += 1;
                } else {
                    tape[pointer] = 0;
                }
            },
            b'[' => {
                if tape[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets != 0 {
                        code_pointer += 1;
                        match code_bytes[code_pointer] {
                            b'[' => open_brackets += 1,
                            b']' => open_brackets -= 1,
                            _ => {},
                        }
                    }
                }
            },
            b']' => {
                if tape[pointer] != 0 {
                    let mut close_brackets = 1;
                    while close_brackets != 0 {
                        code_pointer -= 1;
                        match code_bytes[code_pointer] {
                            b'[' => close_brackets -= 1,
                            b']' => close_brackets += 1,
                            _ => {},
                        }
                    }
                }
            },
            _ => {},
        }
        code_pointer += 1;
    }

    output
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        
        println!("Brainfuck Interpreter");
        println!("Enter your Brainfuck code (end with a line containing only 'END'):");

        let mut code = String::new();
        let mut input = String::new();
        
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            if line.trim() == "END" {
                break;
            }
            code.push_str(&line);
        }

        println!("Enter input data (optional, press Enter to skip):");
        io::stdin().read_line(&mut input)?;

        let output = brainfuck_interpreter(&code, &input.trim());
        println!("Output:\n{}", output);
    } else {
        
        let filename = &args[1];

        let mut file = File::open(filename)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;

        println!("Enter input data (optional, press Enter to skip):");
        let mut input_data = String::new();
        io::stdin().read_line(&mut input_data)?;

        
        let output = brainfuck_interpreter(&code, &input_data.trim());
        println!("Output:\n{}", output);
    }

    Ok(())
}
