use std::fs;
use std::fs::File;
use std::io::prelude::*;
use clap::App;
use clap::Arg;
use clap::ArgGroup;
use log::info;

mod disassembler;
mod emulator;

fn main() {
    let args = App::new("rusty8080")
        .version("0.1.0")
        .author("Andrew Hopkins <andrewjohnhopkins@gmail.com>")
        .about("Emulates programs for the Intel 8080")
        .group(ArgGroup::with_name("mode")
            .args(&["emulate", "disassemble"])
            .required(true))
        .arg(Arg::with_name("emulate")
            .short("e")
            .long("emulate")
            .help("Emulate the program"))
        .arg(Arg::with_name("disassemble")
            .short("d")
            .long("disassemble")
            .help("Disassemble the file for numOps commands"))
        .arg(Arg::with_name("numOps")
            .short("n")
            .long("numOps")
            .default_value("10")
            .help("Number of operations to disassemble"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("PATH_TO_FILE")
            .required(true)
            .help("The file to emulate")
            .takes_value(true))
        .arg(Arg::with_name("logLevel")
            .short("l")
            .long("logLevel")
            .value_name("LEVEL")
            .default_value("debug")
            .possible_values(["debug", "info", "error"].as_ref())
            .help("Sets the level of logging"))
        .get_matches();

    let filename = args.value_of("file").unwrap();

    if args.is_present("emulate") {
        emulate(filename);
    } else {
        let num_bytes = args.value_of("numOps").unwrap().parse::<usize>().unwrap_or(10);
        disassemle(filename, num_bytes);
    }
}

fn disassemle(filename: &str, requested_bytes: usize) {
    info!("Opening: {}", filename);
    let contents = fs::read(filename)
        .expect("Could not open file");
    let mut program_counter: usize = 0;
    while program_counter < requested_bytes && program_counter < contents.len() {
        let (code, byes_used) = disassembler::disassemble_8080_op(&contents, program_counter);
        program_counter += byes_used;
        println!("{}", code);
    }
}

fn emulate(filename: &str) {
    info!("Opening: {}", filename);
    let mut game_file = File::open(filename).expect("Failed to open file!");

    // 8080 has 64 KB of memory
    let mut game_memory = vec![0; 64_000];

    // Load the game into the game_memory starting at 0
    game_file.read_to_end(&mut game_memory).expect("Failed to read game into memory!");

    let mut state = emulator::State8080::new(game_memory);
    emulator::emulate_op(&mut state);
}