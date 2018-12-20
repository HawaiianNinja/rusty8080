# rusty8080
A emulator for the 8080 CPU written in Rust. Based on the guide from emulator101.com. Intel 8080 manual from http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf is also handy.

# Example
```
cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rusty8080 -h`
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
    -l, --logLevel <LEVEL>       Sets the level of logging [default: debug]  [possible values: debug, info, error]
    -n, --numOps <numOps>        Number of operations to disassemble [default: 10]
```