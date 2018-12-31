use clap::App;
use clap::Arg;
use clap::ArgGroup;
use log::info;
use piston_window::*;
use std::fs;

mod disassembler;
mod emulator;

fn main() {
    let args = App::new("rusty8080")
        .version("0.1.0")
        .author("Andrew Hopkins <andrewjohnhopkins@gmail.com>")
        .about("Emulates programs for the Intel 8080")
        .group(
            ArgGroup::with_name("mode")
                .args(&["emulate", "disassemble"])
                .required(true),
        )
        .arg(
            Arg::with_name("emulate")
                .short("e")
                .long("emulate")
                .help("Emulate the program"),
        )
        .arg(
            Arg::with_name("disassemble")
                .short("d")
                .long("disassemble")
                .help("Disassemble the file for numOps commands"),
        )
        .arg(
            Arg::with_name("numOps")
                .short("n")
                .long("numOps")
                .default_value("10")
                .help("Number of operations"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("PATH_TO_FILE")
                .required(true)
                .help("The file to emulate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("logFile")
                .short("l")
                .long("logFile")
                .value_name("FILE")
                .help("Sets the log config"),
        )
        .get_matches();

    let log_file = args.value_of("logFile").unwrap();

    log4rs::init_file(log_file, Default::default()).unwrap();

    let filename = args.value_of("file").unwrap();
    let num_operations = args
        .value_of("numOps")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(10);

    if args.is_present("emulate") {
        emulate(filename, num_operations);
    } else {
        disassemble(filename, num_operations);
    }
}

fn disassemble(filename: &str, requested_bytes: usize) {
    info!("Opening: {}", filename);
    let contents = fs::read(filename).expect("Could not open file");
    let mut program_counter: usize = 0;
    while program_counter < requested_bytes && program_counter < contents.len() {
        let (code, byes_used) = disassembler::disassemble_op(&contents, program_counter);
        program_counter += byes_used;
        println!("{}", code);
    }
}

fn emulate(filename: &str, num_operations: usize) {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
    info!("Opening: {}", filename);
    let mut game_memory = fs::read(filename).expect("Could not open file");
    game_memory.resize(64_000, 0);

    let mut state = emulator::State8080::new(game_memory);
    for _ in 0..num_operations {
        state.emulate_op();
    }
}
