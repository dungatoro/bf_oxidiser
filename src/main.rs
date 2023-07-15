use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};

// itertools coalesce

#[derive(Debug)]
enum Commands {
    Move(usize, u32),
    Add(usize, u32),
    Print,
    Input,
    Loop,
    EndLoop,
}

fn assemble(bf_source: Vec<u8>) -> Vec<Commands> {
    let mut program_code: Vec<Commands> = vec![];
    let mut i: usize = 0;
    while i < bf_source.len() {
        let mut inc: u32 = 0;
        match bf_source[i] {
            b'>' => { loop { if bf_source[i] == b'>' { inc += 1} else if bf_source[i] == b'<' { inc += 29999} else { break;} i += 1}
                      program_code.push(Commands::Move(0, inc))},
            b'<' => { loop { if bf_source[i] == b'>' { inc += 1} else if bf_source[i] == b'<' { inc += 29999} else { break;} i += 1}
                      program_code.push(Commands::Move(0, inc))},
                       
            b'+' => { loop { if bf_source[i] == b'+' { inc += 1} else if bf_source[i] == b'-' { inc += 255} else { break;} i += 1}
                      program_code.push(Commands::Add(0, inc))},
            b'-' => { loop { if bf_source[i] == b'+' { inc += 1} else if bf_source[i] == b'-' { inc += 255} else { break;} i += 1}
                      program_code.push(Commands::Add(0, inc))},
                       
            b'.' => { i += 1; program_code.push(Commands::Print)},
            b',' => { i += 1; program_code.push(Commands::Input)},
             
            b'[' => { i += 1; program_code.push(Commands::Loop)},
            b']' => { i += 1; program_code.push(Commands::EndLoop)},
            _ => i += 1,
        }
    }

    program_code
}

fn optimise(program_code: Vec<Commands>) -> Vec<Commands> {
    let optimised_code: Vec<Commands> = vec![];
    optimised_code
}

fn compile_write(filename: String, program_code: Vec<Commands>) {
    let _ = create_dir_all("oxidised/src/".to_owned()+&filename.clone()+"").expect("Cannot create dir");
    let mut rs_file = OpenOptions::new() .create(true) .append(true)
        .open("oxidised/src/".to_owned()+&filename.clone()+"/mod.rs")
        .expect("cannot create file");

    writeln!(rs_file, "pub fn {}() {{\n\nlet mut tape: [u8;30000] = [0;30000];\nlet mut p: usize = 0;\n", filename)
        .expect("Could not write to file");

    for c in program_code {
        match c {
            Commands::Move(_, n) => { writeln!(rs_file, "p += {}; p %= 30000; ", n).unwrap();},
            Commands::Add(_, n)  => { writeln!(rs_file, "tape[p] = tape[p].wrapping_add({});", n % 256).unwrap();},

            Commands::Print   => {writeln!(rs_file, r#"print!("{{}}", tape[p] as char);"#).unwrap(); 
                                  writeln!(rs_file, "").unwrap();},
            Commands::Input   => {writeln!(rs_file, r#"tape[p] = {{let mut input: [u8; 1] = [0; 1]; std::io::stdin().read_exact(&mut input).expect(""); input[0]}}"#).unwrap();
                                  writeln!(rs_file, "").unwrap();},

            Commands::Loop    => {writeln!(rs_file, "\nwhile tape[p] != 0 {{").unwrap();},
            Commands::EndLoop => {writeln!(rs_file, "}}\n").unwrap();},
        }
    }

    writeln!(rs_file, "}}").unwrap();
}

fn main() {
    let filename = String::from("calc");

    let mut bf_file = File::open(filename.clone()+".bf").expect("Unable to read file.");
    let mut bf_source = Vec::new();
    bf_file.read_to_end(&mut bf_source).unwrap();

    let program_code = assemble(bf_source);
    println!("{:?}", program_code);
    // let program_code = optimise(program_code);

    compile_write(filename, program_code);
}

