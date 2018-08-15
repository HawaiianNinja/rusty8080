
pub fn disassemble_8080_op(buff: &Vec<u8>, pc: usize) -> (String, usize) {
    let mut result = format!("{:04x} ", pc);
    let bytes_used: usize;

    let code = buff.get(pc)
        .expect(&format!("Failed to read buffer at {}", pc));

    match code {
        0x00 => { result +=          "NOP"; bytes_used = 1;}
        0x01 => { result += &format!("LXI    B,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x02 => { result +=          "STAX   B"; bytes_used = 1; }
        0x03 => { result +=          "INX    B"; bytes_used = 1; }
        0x04 => { result +=          "INR    B"; bytes_used = 1; }
        0x05 => { result +=          "DCR    B"; bytes_used = 1; }
        0x06 => { result += &format!("MVI    B,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x07 => { result +=          "RLC"; bytes_used = 1; }
        0x08 => { result +=          "NOP"; bytes_used = 1; }
        0x09 => { result +=          "DAD    B"; bytes_used = 1; }
        0x0a => { result +=          "LDAX   B"; bytes_used = 1; }
        0x0b => { result +=          "DCX    B"; bytes_used = 1; }
        0x0c => { result +=          "INR    C"; bytes_used = 1; }
        0x0d => { result +=          "DCR    C"; bytes_used = 1; }
        0x0e => { result += &format!("MVI    C,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x0f => { result +=          "RRC"; bytes_used = 1; }

        0x10 => { result +=          "NOP"; bytes_used = 1; }
        0x11 => { result += &format!("LXI    D,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x12 => { result +=          "STAX   D"; bytes_used = 1; }
        0x13 => { result +=          "INX    D"; bytes_used = 1; }
        0x14 => { result +=          "INR    D"; bytes_used = 1; }
        0x15 => { result +=          "DCR    D"; bytes_used = 1; }
        0x16 => { result += &format!("MVI    D,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x17 => { result +=          "RAL"; bytes_used = 1; }
        0x18 => { result +=          "NOP"; bytes_used = 1; }
        0x19 => { result +=          "DAD    D"; bytes_used = 1; }
        0x1a => { result +=          "LDAX   D"; bytes_used = 1; }
        0x1b => { result +=          "DCX    D"; bytes_used = 1; }
        0x1c => { result +=          "INR    E"; bytes_used = 1; }
        0x1d => { result +=          "DCR    E"; bytes_used = 1; }
        0x1e => { result += &format!("MVI    E,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x1f => { result +=          "RAR"; bytes_used = 1; }

        0x20 => { result +=          "RIM"; bytes_used = 1; }
        0x21 => { result += &format!("LXI    H,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x22 => { result += &format!("SHLD   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x23 => { result +=          "INX    H"; bytes_used = 1; }
        0x24 => { result +=          "INR    H"; bytes_used = 1; }
        0x25 => { result +=          "DCR    H"; bytes_used = 1; }
        0x26 => { result += &format!("MVI    H,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x27 => { result +=          "DAA"; bytes_used = 1; }
        0x28 => { result +=          "NOP"; bytes_used = 1; }
        0x29 => { result +=          "DAD    H"; bytes_used = 1; }
        0x2a => { result += &format!("LHLD   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x2b => { result +=          "DCX    H"; bytes_used = 1; }
        0x2c => { result +=          "INR    L"; bytes_used = 1; }
        0x2d => { result +=          "DCR    L"; bytes_used = 1; }
        0x2e => { result += &format!("MVI    L,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x2f => { result +=          "CMA"; bytes_used = 1; }

        0x30 => { result +=          "SIM"; bytes_used = 1; }
        0x31 => { result += &format!("LXI    SP,#${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x32 => { result += &format!("STA    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x33 => { result +=          "INX    SP"; bytes_used = 1; }
        0x34 => { result +=          "INR    M"; bytes_used = 1; }
        0x35 => { result +=          "DCR    M"; bytes_used = 1; }
        0x36 => { result += &format!("MVI    M,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x37 => { result +=          "STC"; bytes_used = 1; }
        0x38 => { result +=          "NOP"; bytes_used = 1; }
        0x39 => { result +=          "DAD    SP"; bytes_used = 1; }
        0x3a => { result += &format!("LDA    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0x3b => { result +=          "DCX    SP"; bytes_used = 1; }
        0x3c => { result +=          "INR    A"; bytes_used = 1; }
        0x3d => { result +=          "DCR    A"; bytes_used = 1; }
        0x3e => { result += &format!("MVI    A,#${:02x}", buff[pc + 1]); bytes_used = 2; }
        0x3f => { result +=          "CMC"; bytes_used = 1; }

        0x40 => { result +=          "MOV    B,B"; bytes_used = 1; }
        0x41 => { result +=          "MOV    B,C"; bytes_used = 1; }
        0x42 => { result +=          "MOV    B,D"; bytes_used = 1; }
        0x43 => { result +=          "MOV    B,E"; bytes_used = 1; }
        0x44 => { result +=          "MOV    B,H"; bytes_used = 1; }
        0x45 => { result +=          "MOV    B,L"; bytes_used = 1; }
        0x46 => { result +=          "MOV    B,M"; bytes_used = 1; }
        0x47 => { result +=          "MOV    B,A"; bytes_used = 1; }
        0x48 => { result +=          "MOV    C,B"; bytes_used = 1; }
        0x49 => { result +=          "MOV    C,C"; bytes_used = 1; }
        0x4a => { result +=          "MOV    C,D"; bytes_used = 1; }
        0x4b => { result +=          "MOV    C,E"; bytes_used = 1; }
        0x4c => { result +=          "MOV    C,H"; bytes_used = 1; }
        0x4d => { result +=          "MOV    C,L"; bytes_used = 1; }
        0x4e => { result +=          "MOV    C,M"; bytes_used = 1; }
        0x4f => { result +=          "MOV    C,A"; bytes_used = 1; }

        0x50 => { result +=          "MOV    D,B"; bytes_used = 1; }
        0x51 => { result +=          "MOV    D,C"; bytes_used = 1; }
        0x52 => { result +=          "MOV    D,D"; bytes_used = 1; }
        0x53 => { result +=          "MOV    D,E"; bytes_used = 1; }
        0x54 => { result +=          "MOV    D,H"; bytes_used = 1; }
        0x55 => { result +=          "MOV    D,L"; bytes_used = 1; }
        0x56 => { result +=          "MOV    D,M"; bytes_used = 1; }
        0x57 => { result +=          "MOV    D,A"; bytes_used = 1; }
        0x58 => { result +=          "MOV    E,B"; bytes_used = 1; }
        0x59 => { result +=          "MOV    E,C"; bytes_used = 1; }
        0x5a => { result +=          "MOV    E,D"; bytes_used = 1; }
        0x5b => { result +=          "MOV    E,E"; bytes_used = 1; }
        0x5c => { result +=          "MOV    E,H"; bytes_used = 1; }
        0x5d => { result +=          "MOV    E,L"; bytes_used = 1; }
        0x5e => { result +=          "MOV    E,M"; bytes_used = 1; }
        0x5f => { result +=          "MOV    E,A"; bytes_used = 1; }

        0x60 => { result +=          "MOV    H,B"; bytes_used = 1; }
        0x61 => { result +=          "MOV    H,C"; bytes_used = 1; }
        0x62 => { result +=          "MOV    H,D"; bytes_used = 1; }
        0x63 => { result +=          "MOV    H,E"; bytes_used = 1; }
        0x64 => { result +=          "MOV    H,H"; bytes_used = 1; }
        0x65 => { result +=          "MOV    H,L"; bytes_used = 1; }
        0x66 => { result +=          "MOV    H,M"; bytes_used = 1; }
        0x67 => { result +=          "MOV    H,A"; bytes_used = 1; }
        0x68 => { result +=          "MOV    L,B"; bytes_used = 1; }
        0x69 => { result +=          "MOV    L,C"; bytes_used = 1; }
        0x6a => { result +=          "MOV    L,D"; bytes_used = 1; }
        0x6b => { result +=          "MOV    L,E"; bytes_used = 1; }
        0x6c => { result +=          "MOV    L,H"; bytes_used = 1; }
        0x6d => { result +=          "MOV    L,L"; bytes_used = 1; }
        0x6e => { result +=          "MOV    L,M"; bytes_used = 1; }
        0x6f => { result +=          "MOV    L,A"; bytes_used = 1; }

        0x70 => { result +=          "MOV    M,B"; bytes_used = 1; }
        0x71 => { result +=          "MOV    M,C"; bytes_used = 1; }
        0x72 => { result +=          "MOV    M,D"; bytes_used = 1; }
        0x73 => { result +=          "MOV    M,E"; bytes_used = 1; }
        0x74 => { result +=          "MOV    M,H"; bytes_used = 1; }
        0x75 => { result +=          "MOV    M,L"; bytes_used = 1; }
        0x76 => { result +=          "MOV    M,M"; bytes_used = 1; }
        0x77 => { result +=          "HLT"; bytes_used = 1; }
        0x78 => { result +=          "MOV    A,B"; bytes_used = 1; }
        0x79 => { result +=          "MOV    A,C"; bytes_used = 1; }
        0x7a => { result +=          "MOV    A,D"; bytes_used = 1; }
        0x7b => { result +=          "MOV    A,E"; bytes_used = 1; }
        0x7c => { result +=          "MOV    A,H"; bytes_used = 1; }
        0x7d => { result +=          "MOV    A,L"; bytes_used = 1; }
        0x7e => { result +=          "MOV    A,M"; bytes_used = 1; }
        0x7f => { result +=          "MOV    A,A"; bytes_used = 1; }

        0x80 => { result +=          "ADD    B"; bytes_used = 1; }
        0x81 => { result +=          "ADD    C"; bytes_used = 1; }
        0x82 => { result +=          "ADD    D"; bytes_used = 1; }
        0x83 => { result +=          "ADD    E"; bytes_used = 1; }
        0x84 => { result +=          "ADD    H"; bytes_used = 1; }
        0x85 => { result +=          "ADD    L"; bytes_used = 1; }
        0x86 => { result +=          "ADD    M"; bytes_used = 1; }
        0x87 => { result +=          "ADD    A"; bytes_used = 1; }
        0x88 => { result +=          "ADC    B"; bytes_used = 1; }
        0x89 => { result +=          "ADC    C"; bytes_used = 1; }
        0x8a => { result +=          "ADC    D"; bytes_used = 1; }
        0x8b => { result +=          "ADC    E"; bytes_used = 1; }
        0x8c => { result +=          "ADC    H"; bytes_used = 1; }
        0x8d => { result +=          "ADC    L"; bytes_used = 1; }
        0x8e => { result +=          "ADC    M"; bytes_used = 1; }
        0x8f => { result +=          "ADC    A"; bytes_used = 1; }

        0x90 => { result +=          "SUB    B"; bytes_used = 1; }
        0x91 => { result +=          "SUB    C"; bytes_used = 1; }
        0x92 => { result +=          "SUB    D"; bytes_used = 1; }
        0x93 => { result +=          "SUB    E"; bytes_used = 1; }
        0x94 => { result +=          "SUB    H"; bytes_used = 1; }
        0x95 => { result +=          "SUB    L"; bytes_used = 1; }
        0x96 => { result +=          "SUB    M"; bytes_used = 1; }
        0x97 => { result +=          "SUB    A"; bytes_used = 1; }
        0x98 => { result +=          "SBB    B"; bytes_used = 1; }
        0x99 => { result +=          "SBB    C"; bytes_used = 1; }
        0x9a => { result +=          "SBB    D"; bytes_used = 1; }
        0x9b => { result +=          "SBB    E"; bytes_used = 1; }
        0x9c => { result +=          "SBB    H"; bytes_used = 1; }
        0x9d => { result +=          "SBB    L"; bytes_used = 1; }
        0x9e => { result +=          "SBB    M"; bytes_used = 1; }
        0x9f => { result +=          "SBB    A"; bytes_used = 1; }

        0xa0 => { result +=          "ANA    B"; bytes_used = 1; }
        0xa1 => { result +=          "ANA    C"; bytes_used = 1; }
        0xa2 => { result +=          "ANA    D"; bytes_used = 1; }
        0xa3 => { result +=          "ANA    E"; bytes_used = 1; }
        0xa4 => { result +=          "ANA    H"; bytes_used = 1; }
        0xa5 => { result +=          "ANA    L"; bytes_used = 1; }
        0xa6 => { result +=          "ANA    M"; bytes_used = 1; }
        0xa7 => { result +=          "ANA    A"; bytes_used = 1; }
        0xa8 => { result +=          "XRA    B"; bytes_used = 1; }
        0xa9 => { result +=          "XRA    C"; bytes_used = 1; }
        0xaa => { result +=          "XRA    D"; bytes_used = 1; }
        0xab => { result +=          "XRA    E"; bytes_used = 1; }
        0xac => { result +=          "XRA    H"; bytes_used = 1; }
        0xad => { result +=          "XRA    L"; bytes_used = 1; }
        0xae => { result +=          "XRA    M"; bytes_used = 1; }
        0xaf => { result +=          "XRA    A"; bytes_used = 1; }

        0xb0 => { result +=          "ORA    B"; bytes_used = 1; }
        0xb1 => { result +=          "ORA    C"; bytes_used = 1; }
        0xb2 => { result +=          "ORA    D"; bytes_used = 1; }
        0xb3 => { result +=          "ORA    E"; bytes_used = 1; }
        0xb4 => { result +=          "ORA    H"; bytes_used = 1; }
        0xb5 => { result +=          "ORA    L"; bytes_used = 1; }
        0xb6 => { result +=          "ORA    M"; bytes_used = 1; }
        0xb7 => { result +=          "ORA    A"; bytes_used = 1; }
        0xb8 => { result +=          "CMP    B"; bytes_used = 1; }
        0xb9 => { result +=          "CMP    C"; bytes_used = 1; }
        0xba => { result +=          "CMP    D"; bytes_used = 1; }
        0xbb => { result +=          "CMP    E"; bytes_used = 1; }
        0xbc => { result +=          "CMP    H"; bytes_used = 1; }
        0xbd => { result +=          "CMP    L"; bytes_used = 1; }
        0xbe => { result +=          "CMP    M"; bytes_used = 1; }
        0xbf => { result +=          "CMP    A"; bytes_used = 1; }

        0xc0 => { result +=          "RNZ"; bytes_used = 1; }
        0xc1 => { result +=          "POP    B"; bytes_used = 1; }
        0xc2 => { result += &format!("JNZ    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc3 => { result += &format!("JMP    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc4 => { result += &format!("CNZ    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xc5 => { result +=          "PUSH   B"; bytes_used = 1; }
        0xc6 => { result += &format!("ADI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xc7 => { result +=          "RST    0"; bytes_used = 1; }
        0xc8 => { result +=          "RZ"; bytes_used = 1; }
        0xc9 => { result +=          "RET"; bytes_used = 1; }
        0xca => { result += &format!("JZ     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xcb => { result +=          "NOP"; bytes_used = 1; }
        0xcc => { result += &format!("CZ     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xcd => { result += &format!("CALL   ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xce => { result += &format!("ACI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xcf => { result +=          "RST    1"; bytes_used = 1; }

        0xd0 => { result +=          "RNC"; bytes_used = 1; }
        0xd1 => { result +=          "POP    D"; bytes_used = 1; }
        0xd2 => { result += &format!("JNC    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xd3 => { result += &format!("OUT    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xd4 => { result += &format!("CNC    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xd5 => { result +=          "PUSH   D"; bytes_used = 1; }
        0xd6 => { result += &format!("SUI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xd7 => { result +=          "RST    2"; bytes_used = 1; }
        0xd8 => { result +=          "RC"; bytes_used = 1; }
        0xd9 => { result +=          "NOP"; bytes_used = 1; }
        0xda => { result += &format!("JC     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xdb => { result += &format!("IN     #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xdc => { result += &format!("CC     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xdd => { result +=          "NOP"; bytes_used = 1; }
        0xde => { result += &format!("SBI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xdf => { result +=          "RST    3"; bytes_used = 1; }

        0xe0 => { result +=          "RPO"; bytes_used = 1; }
        0xe1 => { result +=          "POP    H"; bytes_used = 1; }
        0xe2 => { result += &format!("JPO    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xe3 => { result +=          "XTHL"; bytes_used = 1; }
        0xe4 => { result += &format!("CPO    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xe5 => { result +=          "PUSH   H"; bytes_used = 1; }
        0xe6 => { result += &format!("ANI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xe7 => { result +=          "RST    4"; bytes_used = 1; }
        0xe8 => { result +=          "RPE"; bytes_used = 1; }
        0xe9 => { result +=          "PCHL"; bytes_used = 1; }
        0xea => { result += &format!("JPE    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xeb => { result +=          "XCHG"; bytes_used = 1; }
        0xec => { result += &format!("CPE    ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xed => { result +=          "NOP"; bytes_used = 1; }
        0xee => { result += &format!("XRI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xef => { result +=          "RST    5"; bytes_used = 1; }

        0xf0 => { result +=          "RP"; bytes_used = 1; }
        0xf1 => { result +=          "POP    PSW"; bytes_used = 1; }
        0xf2 => { result += &format!("JP     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xf3 => { result +=          "DI"; bytes_used = 1; }
        0xf4 => { result += &format!("CP     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xf5 => { result +=          "PUSH   PSW"; bytes_used = 1; }
        0xf6 => { result += &format!("ORI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xf7 => { result +=          "RST    6"; bytes_used = 1; }
        0xf8 => { result +=          "RIM"; bytes_used = 1; }
        0xf9 => { result +=          "SPHL"; bytes_used = 1; }
        0xfa => { result += &format!("JM     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xfb => { result +=          "EI"; bytes_used = 1; }
        0xfc => { result += &format!("CM     ${:02x}{:02x}", buff[pc + 2], buff[pc + 1]); bytes_used = 3; }
        0xfd => { result +=          "NOP"; bytes_used = 1; }
        0xfe => { result += &format!("CPI    #${:02x}", buff[pc + 1]); bytes_used = 2; }
        0xff => { result +=          "RST    7"; bytes_used = 1; }

        _ => { panic!("Unkown op code {:02x} ", code); }
    }

    return (result, bytes_used);
}