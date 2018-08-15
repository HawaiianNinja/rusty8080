use std::env;
use std::fs;

mod disassembler;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 { return; }
    let operation = &args[1];
    match operation.as_ref() {
        "dis" => {
            if args.len() <= 1 {println!("Expected file name"); return; }
            let filename = &args[2];
            let requested_bytes : usize = if args.len() <= 3 { 10 } else { let x =  &args[3].parse::<usize>().unwrap_or(10); *x };

            disassemle_all(filename, requested_bytes);
        }
        _ => { println!("{} is not valid", operation)}
    }
}

fn disassemle_all(filename: &String, requested_bytes: usize) -> () {
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


