// register
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum REG {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT
}

impl REG {
    pub fn new(reg: u16) -> Result<REG, &'static str> {
        Ok(match reg {
            0 => REG::R0,
            1 => REG::R1,
            2 => REG::R2,
            3 => REG::R3,
            4 => REG::R4,
            5 => REG::R5,
            6 => REG::R6,
            7 => REG::R7,

            8 => REG::PC,
            9 => REG::COND,
            _ => return Err("invalid register")
        })
    }
}

impl From<u16> for REG {
    fn from(reg: u16) -> REG {
        REG::new(reg).expect("invalid register")
    }
}

// operation codes
pub enum OP {
    BR   = 0b0000,  // branch
    ADD  = 0b0001,  // add
    LD   = 0b0010,  // load
    ST   = 0b0011,  // store
    JSR  = 0b0100,  // jump register
    AND  = 0b0101,  // bitwise and
    LDR  = 0b0110,  // load register
    STR  = 0b0111,  // store register
    RTI  = 0b1000,  // unused
    NOT  = 0b1001,  // bitwise not
    LDI  = 0b1010,  // load indirect
    STI  = 0b1011,  // store indirect
    JMP  = 0b1100,  // jump
    RES  = 0b1101,  // reserved (unused)
    LEA  = 0b1110,  // load effective address
    TRAP = 0b1111   // execute trap
}

impl OP {
    pub fn new(op: u16) -> Result<OP, &'static str> {
        Ok(match op {
            0b0000 => OP::BR,
            0b0001 => OP::ADD,
            0b0010 => OP::LD,
            0b0011 => OP::ST,
            0b0100 => OP::JSR,
            0b0101 => OP::AND,
            0b0110 => OP::LDR,
            0b0111 => OP::STR,
            0b1000 => OP::RTI,
            0b1001 => OP::NOT,
            0b1010 => OP::LDI,
            0b1011 => OP::STI,
            0b1100 => OP::JMP,
            0b1101 => OP::RES,
            0b1110 => OP::LEA,
            0b1111 => OP::TRAP,
            _ => return Err("invalid operation")
        })
    }
}

// flags
pub enum FL {
    POS = 1 << 0,  // P
    ZRO = 1 << 1,  // Z
    NEG = 1 << 2,  // N
}
