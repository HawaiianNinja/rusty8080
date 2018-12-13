use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod disassembler;
mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please specify operation to run: 'dis' or 'emu'!");
        return;
    }
    let operation = &args[1];
    match operation.as_ref() {
        "dis" => {
            if args.len() <= 2 {println!("Expected file name"); return; }
            let filename = &args[2];
            let requested_bytes : usize = if args.len() <= 3 { 10 } else { let x =  &args[3].parse::<usize>().unwrap_or(10); *x };

            disassemle(filename, requested_bytes);
        }
        "emu" => {
            if args.len() <= 2 {println!("Expected file name"); return; }
            let filename = &args[2];
            emulate(filename);
        }
        _ => { println!("{} is not valid", operation)}
    }
}

fn disassemle(filename: &String, requested_bytes: usize) {
    println!("Opening: {}", filename);
    let contents = fs::read(filename)
        .expect("Could not open file");
    let mut program_counter: usize = 0;
    while program_counter < requested_bytes && program_counter < contents.len() {
        let (code, byes_used) = disassembler::disassemble_8080_op(&contents, program_counter);
        program_counter += byes_used;
        println!("{}", code);
    }
}

fn emulate(filename: &String) {
    println!("Opening: {}", filename);
    let mut game_file = File::open(filename).expect("Failed to open file!");

    // 8080 has 64 KB of memory
    let mut game_memory = vec![0; 64_000];

    // Load the game into the game_memory starting at 0
    game_file.read_to_end(&mut game_memory).expect("Failed to read game into memory!");

    let mut state = emulator::State8080::new(game_memory);
    emulator::emulate_op(&mut state);
}