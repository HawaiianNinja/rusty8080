# rusty8080
A emulator for the 8080 CPU written in Rust. Based on the guide from emulator101.com. Intel 8080 manual from http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf is also handy.

# Example
```
cargo run -- -h
rusty8080 0.1.0
Andrew Hopkins <andrewjohnhopkins@gmail.com>
Emulates programs for the Intel 8080

USAGE:
    rusty8080 [OPTIONS] --file <PATH_TO_FILE> <--emulate|--disassemble>

FLAGS:
    -d, --disassemble    Disassemble the file for numOps commands
    -e, --emulate        Emulate the program
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -f, --file <PATH_TO_FILE>    The file to emulate
    -l, --logFile <FILE>         Sets the log config
    -n, --numOps <numOps>        Number of operations to disassemble [default: 10]

cargo run -- -e -f invaders.atari -l log4rs.yaml
```