use lib::cpu::RegBank;
use lib::memory::Memory;
use lib::reference::REG;

use std::io;
use std::io::{Read, Write};


fn trap_out(reg: &mut RegBank, _mem: &mut Memory) -> io::Result<()> {
    let c = reg.read(REG::R0) as u8;
    print!("{}", c as char);
    io::stdout().flush()?;
    Ok(())
}

fn trap_puts(reg: &mut RegBank, mem: &mut Memory) -> io::Result<()> {
    let mut read_char = |p: u16| {
        let c = mem.read(p) as u8;
        if c != 0 {
            Some(c as char)
        } else {
            None
        }
    };

    let mut ptr = reg.read(REG::R0);

    while let Some(c) = read_char(ptr) {
        print!("{}", c);
        ptr += 1;
    }

    io::stdout().flush()
}

fn trap_putsp(reg: &mut RegBank, mem: &mut Memory) -> io::Result<()> {
    let mut read_char = |p: u16| {
        let word = mem.read(p);
        let c1 = word as u8;
        let c2 = (word >> 8) as u8;

        if c1 > 0 {
            Some((c1, c2))
        } else {
            None
        }
    };

    let mut ptr = reg.read(REG::R0);

    while let Some((c1, c2)) = read_char(ptr) {
        print!("{}", c1 as char);
        if c2 > 0 {
            print!("{}", c2 as char);
        } else {
            break;
        }

        ptr += 1;
    }

    io::stdout().flush()
}

fn trap_getc(reg: &mut RegBank, _mem: &mut Memory) -> io::Result<()> {
    let mut buffer: [u8; 1] = [0];
    let stdin = io::stdin();
    stdin.lock().read_exact(&mut buffer)?;

    reg.write(REG::R0, buffer[0] as u16);
    Ok(())
}

fn trap_in(reg: &mut RegBank, mem: &mut Memory) -> io::Result<()> {
    print!("Input: ");
    io::stdout().flush()?;
    trap_getc(reg, mem)
}


fn trap_halt(_reg: &mut RegBank, _mem: &mut Memory) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "HALT"))
}


use std::collections::HashMap;
pub type Trap = fn(&mut RegBank, &mut Memory) -> io::Result<()>;
pub type TrapMap = HashMap<u8, Box<Trap>>;

pub fn build_trap_vec() -> TrapMap {
    let mut traps = TrapMap::new();

    traps.insert(0x20, Box::new(trap_getc));
    traps.insert(0x21, Box::new(trap_out));
    traps.insert(0x22, Box::new(trap_puts));
    traps.insert(0x23, Box::new(trap_in));
    traps.insert(0x24, Box::new(trap_putsp));
    traps.insert(0x25, Box::new(trap_halt));

    traps
}
