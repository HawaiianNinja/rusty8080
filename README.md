# rusty8080
A emulator for the 8080 CPU written in Rust. Based on the guide from [Emulator 101](http://emulator101.com). Handy links:
* [Intel 8080 programming manual](http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
* [Javascript emulator](https://bluishcoder.co.nz/js8080/)
* [C emulator source code](https://github.com/kpmiller/emulator101)

# Usage
```
$ cargo run -- -e -f invaders.atari -l log4rs.yaml
Opening: invaders.atari
00 NOP              a:00 bc:0000 de:0000 hl:0000 pc:0000 sp:f000 ......
...

$ cargo run -- -h
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
```
