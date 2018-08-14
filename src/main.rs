use std::env;
use std::fs;

mod disassembler;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Opening: {}", filename);
    let contents = fs::read(filename)
        .expect("Could not open file");

    let mut program_counter: usize = 0;
    while program_counter < 10 {
        program_counter += disassembler::disassemble_8080_op(&contents, program_counter);
    }
}
