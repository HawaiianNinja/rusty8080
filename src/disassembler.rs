
pub fn disassemble_8080_op(buffer: &Vec<u8>, program_counter: usize) -> usize {
    print!("{:04x} ", program_counter);

    let code = buffer.get(program_counter)
        .expect(&format!("Failed to read buffer at {}", program_counter));

    match code {
        0x00 => { print!("NOP"); }
        _ => { print!("{:04x} ", code); }
    }


    print!("\n");
    return 1;
}