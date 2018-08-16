
pub fn disassemble_8080_op(buff: &Vec<u8>, pc: usize) -> (String, usize) {
    let mut result = format!("{:04x} ", pc);
    let mut bytes_used = 1;

    let code = buff.get(pc)
        .expect(&format!("Failed to read buffer at {}", pc));

    match code {
        0x00 => { result +=          "NOP"; }
        0x01 => { result += &format!("LXI    B,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x02 => { result +=          "STAX   B"; }
        0x03 => { result +=          "INX    B"; }
        0x04 => { result +=          "INR    B"; }
        0x05 => { result +=          "DCR    B"; }
        0x06 => { result += &format!("MVI    B,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x07 => { result +=          "RLC"; }
        0x08 => { result +=          "NOP"; }
        0x09 => { result +=          "DAD    B"; }
        0x0a => { result +=          "LDAX   B"; }
        0x0b => { result +=          "DCX    B"; }
        0x0c => { result +=          "INR    C"; }
        0x0d => { result +=          "DCR    C"; }
        0x0e => { result += &format!("MVI    C,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x0f => { result +=          "RRC"; }

        0x10 => { result +=          "NOP"; }
        0x11 => { result += &format!("LXI    D,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x12 => { result +=          "STAX   D"; }
        0x13 => { result +=          "INX    D"; }
        0x14 => { result +=          "INR    D"; }
        0x15 => { result +=          "DCR    D"; }
        0x16 => { result += &format!("MVI    D,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x17 => { result +=          "RAL"; }
        0x18 => { result +=          "NOP"; }
        0x19 => { result +=          "DAD    D"; }
        0x1a => { result +=          "LDAX   D"; }
        0x1b => { result +=          "DCX    D"; }
        0x1c => { result +=          "INR    E"; }
        0x1d => { result +=          "DCR    E"; }
        0x1e => { result += &format!("MVI    E,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x1f => { result +=          "RAR"; }

        0x20 => { result +=          "RIM"; }
        0x21 => { result += &format!("LXI    H,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x22 => { result += &format!("SHLD   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x23 => { result +=          "INX    H"; }
        0x24 => { result +=          "INR    H"; }
        0x25 => { result +=          "DCR    H"; }
        0x26 => { result += &format!("MVI    H,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x27 => { result +=          "DAA"; }
        0x28 => { result +=          "NOP"; }
        0x29 => { result +=          "DAD    H"; }
        0x2a => { result += &format!("LHLD   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x2b => { result +=          "DCX    H"; }
        0x2c => { result +=          "INR    L"; }
        0x2d => { result +=          "DCR    L"; }
        0x2e => { result += &format!("MVI    L,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x2f => { result +=          "CMA"; }

        0x30 => { result +=          "SIM"; }
        0x31 => { result += &format!("LXI    SP,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x32 => { result += &format!("STA    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x33 => { result +=          "INX    SP"; }
        0x34 => { result +=          "INR    M"; }
        0x35 => { result +=          "DCR    M"; }
        0x36 => { result += &format!("MVI    M,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x37 => { result +=          "STC"; }
        0x38 => { result +=          "NOP"; }
        0x39 => { result +=          "DAD    SP"; }
        0x3a => { result += &format!("LDA    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x3b => { result +=          "DCX    SP"; }
        0x3c => { result +=          "INR    A"; }
        0x3d => { result +=          "DCR    A"; }
        0x3e => { result += &format!("MVI    A,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x3f => { result +=          "CMC"; }

        0x40 => { result +=          "MOV    B,B"; }
        0x41 => { result +=          "MOV    B,C"; }
        0x42 => { result +=          "MOV    B,D"; }
        0x43 => { result +=          "MOV    B,E"; }
        0x44 => { result +=          "MOV    B,H"; }
        0x45 => { result +=          "MOV    B,L"; }
        0x46 => { result +=          "MOV    B,M"; }
        0x47 => { result +=          "MOV    B,A"; }
        0x48 => { result +=          "MOV    C,B"; }
        0x49 => { result +=          "MOV    C,C"; }
        0x4a => { result +=          "MOV    C,D"; }
        0x4b => { result +=          "MOV    C,E"; }
        0x4c => { result +=          "MOV    C,H"; }
        0x4d => { result +=          "MOV    C,L"; }
        0x4e => { result +=          "MOV    C,M"; }
        0x4f => { result +=          "MOV    C,A"; }

        0x50 => { result +=          "MOV    D,B"; }
        0x51 => { result +=          "MOV    D,C"; }
        0x52 => { result +=          "MOV    D,D"; }
        0x53 => { result +=          "MOV    D,E"; }
        0x54 => { result +=          "MOV    D,H"; }
        0x55 => { result +=          "MOV    D,L"; }
        0x56 => { result +=          "MOV    D,M"; }
        0x57 => { result +=          "MOV    D,A"; }
        0x58 => { result +=          "MOV    E,B"; }
        0x59 => { result +=          "MOV    E,C"; }
        0x5a => { result +=          "MOV    E,D"; }
        0x5b => { result +=          "MOV    E,E"; }
        0x5c => { result +=          "MOV    E,H"; }
        0x5d => { result +=          "MOV    E,L"; }
        0x5e => { result +=          "MOV    E,M"; }
        0x5f => { result +=          "MOV    E,A"; }

        0x60 => { result +=          "MOV    H,B"; }
        0x61 => { result +=          "MOV    H,C"; }
        0x62 => { result +=          "MOV    H,D"; }
        0x63 => { result +=          "MOV    H,E"; }
        0x64 => { result +=          "MOV    H,H"; }
        0x65 => { result +=          "MOV    H,L"; }
        0x66 => { result +=          "MOV    H,M"; }
        0x67 => { result +=          "MOV    H,A"; }
        0x68 => { result +=          "MOV    L,B"; }
        0x69 => { result +=          "MOV    L,C"; }
        0x6a => { result +=          "MOV    L,D"; }
        0x6b => { result +=          "MOV    L,E"; }
        0x6c => { result +=          "MOV    L,H"; }
        0x6d => { result +=          "MOV    L,L"; }
        0x6e => { result +=          "MOV    L,M"; }
        0x6f => { result +=          "MOV    L,A"; }

        0x70 => { result +=          "MOV    M,B"; }
        0x71 => { result +=          "MOV    M,C"; }
        0x72 => { result +=          "MOV    M,D"; }
        0x73 => { result +=          "MOV    M,E"; }
        0x74 => { result +=          "MOV    M,H"; }
        0x75 => { result +=          "MOV    M,L"; }
        0x76 => { result +=          "MOV    M,M"; }
        0x77 => { result +=          "HLT"; }
        0x78 => { result +=          "MOV    A,B"; }
        0x79 => { result +=          "MOV    A,C"; }
        0x7a => { result +=          "MOV    A,D"; }
        0x7b => { result +=          "MOV    A,E"; }
        0x7c => { result +=          "MOV    A,H"; }
        0x7d => { result +=          "MOV    A,L"; }
        0x7e => { result +=          "MOV    A,M"; }
        0x7f => { result +=          "MOV    A,A"; }

        0x80 => { result +=          "ADD    B"; }
        0x81 => { result +=          "ADD    C"; }
        0x82 => { result +=          "ADD    D"; }
        0x83 => { result +=          "ADD    E"; }
        0x84 => { result +=          "ADD    H"; }
        0x85 => { result +=          "ADD    L"; }
        0x86 => { result +=          "ADD    M"; }
        0x87 => { result +=          "ADD    A"; }
        0x88 => { result +=          "ADC    B"; }
        0x89 => { result +=          "ADC    C"; }
        0x8a => { result +=          "ADC    D"; }
        0x8b => { result +=          "ADC    E"; }
        0x8c => { result +=          "ADC    H"; }
        0x8d => { result +=          "ADC    L"; }
        0x8e => { result +=          "ADC    M"; }
        0x8f => { result +=          "ADC    A"; }

        0x90 => { result +=          "SUB    B"; }
        0x91 => { result +=          "SUB    C"; }
        0x92 => { result +=          "SUB    D"; }
        0x93 => { result +=          "SUB    E"; }
        0x94 => { result +=          "SUB    H"; }
        0x95 => { result +=          "SUB    L"; }
        0x96 => { result +=          "SUB    M"; }
        0x97 => { result +=          "SUB    A"; }
        0x98 => { result +=          "SBB    B"; }
        0x99 => { result +=          "SBB    C"; }
        0x9a => { result +=          "SBB    D"; }
        0x9b => { result +=          "SBB    E"; }
        0x9c => { result +=          "SBB    H"; }
        0x9d => { result +=          "SBB    L"; }
        0x9e => { result +=          "SBB    M"; }
        0x9f => { result +=          "SBB    A"; }

        0xa0 => { result +=          "ANA    B"; }
        0xa1 => { result +=          "ANA    C"; }
        0xa2 => { result +=          "ANA    D"; }
        0xa3 => { result +=          "ANA    E"; }
        0xa4 => { result +=          "ANA    H"; }
        0xa5 => { result +=          "ANA    L"; }
        0xa6 => { result +=          "ANA    M"; }
        0xa7 => { result +=          "ANA    A"; }
        0xa8 => { result +=          "XRA    B"; }
        0xa9 => { result +=          "XRA    C"; }
        0xaa => { result +=          "XRA    D"; }
        0xab => { result +=          "XRA    E"; }
        0xac => { result +=          "XRA    H"; }
        0xad => { result +=          "XRA    L"; }
        0xae => { result +=          "XRA    M"; }
        0xaf => { result +=          "XRA    A"; }

        0xb0 => { result +=          "ORA    B"; }
        0xb1 => { result +=          "ORA    C"; }
        0xb2 => { result +=          "ORA    D"; }
        0xb3 => { result +=          "ORA    E"; }
        0xb4 => { result +=          "ORA    H"; }
        0xb5 => { result +=          "ORA    L"; }
        0xb6 => { result +=          "ORA    M"; }
        0xb7 => { result +=          "ORA    A"; }
        0xb8 => { result +=          "CMP    B"; }
        0xb9 => { result +=          "CMP    C"; }
        0xba => { result +=          "CMP    D"; }
        0xbb => { result +=          "CMP    E"; }
        0xbc => { result +=          "CMP    H"; }
        0xbd => { result +=          "CMP    L"; }
        0xbe => { result +=          "CMP    M"; }
        0xbf => { result +=          "CMP    A"; }

        0xc0 => { result +=          "RNZ"; }
        0xc1 => { result +=          "POP    B"; }
        0xc2 => { result += &format!("JNZ    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc3 => { result += &format!("JMP    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc4 => { result += &format!("CNZ    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc5 => { result +=          "PUSH   B"; }
        0xc6 => { result += &format!("ADI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xc7 => { result +=          "RST    0"; }
        0xc8 => { result +=          "RZ"; }
        0xc9 => { result +=          "RET"; }
        0xca => { result += &format!("JZ     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xcb => { result +=          "NOP"; }
        0xcc => { result += &format!("CZ     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xcd => { result += &format!("CALL   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xce => { result += &format!("ACI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xcf => { result +=          "RST    1"; }

        0xd0 => { result +=          "RNC"; }
        0xd1 => { result +=          "POP    D"; }
        0xd2 => { result += &format!("JNC    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xd3 => { result += &format!("OUT    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xd4 => { result += &format!("CNC    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xd5 => { result +=          "PUSH   D"; }
        0xd6 => { result += &format!("SUI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xd7 => { result +=          "RST    2"; }
        0xd8 => { result +=          "RC"; }
        0xd9 => { result +=          "NOP"; }
        0xda => { result += &format!("JC     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xdb => { result += &format!("IN     #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xdc => { result += &format!("CC     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xdd => { result +=          "NOP"; }
        0xde => { result += &format!("SBI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xdf => { result +=          "RST    3"; }

        0xe0 => { result +=          "RPO"; }
        0xe1 => { result +=          "POP    H"; }
        0xe2 => { result += &format!("JPO    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xe3 => { result +=          "XTHL"; }
        0xe4 => { result += &format!("CPO    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xe5 => { result +=          "PUSH   H"; }
        0xe6 => { result += &format!("ANI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xe7 => { result +=          "RST    4"; }
        0xe8 => { result +=          "RPE"; }
        0xe9 => { result +=          "PCHL"; }
        0xea => { result += &format!("JPE    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xeb => { result +=          "XCHG"; }
        0xec => { result += &format!("CPE    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xed => { result +=          "NOP"; }
        0xee => { result += &format!("XRI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xef => { result +=          "RST    5"; }

        0xf0 => { result +=          "RP"; }
        0xf1 => { result +=          "POP    PSW"; }
        0xf2 => { result += &format!("JP     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xf3 => { result +=          "DI"; }
        0xf4 => { result += &format!("CP     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xf5 => { result +=          "PUSH   PSW"; }
        0xf6 => { result += &format!("ORI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xf7 => { result +=          "RST    6"; }
        0xf8 => { result +=          "RIM"; }
        0xf9 => { result +=          "SPHL"; }
        0xfa => { result += &format!("JM     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xfb => { result +=          "EI"; }
        0xfc => { result += &format!("CM     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xfd => { result +=          "NOP"; }
        0xfe => { result += &format!("CPI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xff => { result +=          "RST    7"; }

        _ => { panic!("Unkown op code {:02x} ", code); }
    }

    return (result, bytes_used);
}