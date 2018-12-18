use lib::reference::{REG, FL};
use lib::memory::Memory;
use lib::traps::{TrapMap, build_trap_vec};
use std::error::Error;

pub const PC_START: u16 = 0x3000;

pub struct RegBank {
    bank: [u16; REG::COUNT as usize]
}

impl RegBank {
    pub fn read(&self, reg: REG) -> u16 {
        self.bank[reg as usize]
    }

    pub fn write(&mut self, reg: REG, value: u16) {
        self.bank[reg as usize] = value
    }
}

pub struct CPU<'a> {
    reg: RegBank,
    pub mem: &'a mut Memory,
    trap: TrapMap
}

impl<'a> CPU<'a> {
    pub fn read(&self, reg: REG) -> u16 {
        self.reg.read(reg)
    }

    pub fn write(&mut self, reg: REG, value: u16) {
        self.reg.write(reg, value)
    }

    pub fn new(mem: &'a mut Memory) -> CPU {
        let mut cpu = CPU {
            reg: RegBank {bank: [0; REG::COUNT as usize]},
            mem: mem,
            trap: build_trap_vec()
        };
        cpu.write(REG::PC, PC_START);

        cpu
    }

    pub fn update_flags(&mut self, reg: REG) {
        let value = self.read(reg);
        let sign = value >> 15;

        let flag = if value == 0 {
            FL::ZRO
        } else if sign == 0 {
            FL::POS
        } else {
            FL::NEG
        } as u16;

        self.write(REG::COND, flag)
    }

    pub fn run_trap(&mut self, selector: u8) -> Result<(), &'static str> {
        let trap = match self.trap.get(&selector) {
            Some(trap_box) => &*trap_box,
            None => return Err("Unknown Trap")
        };

        match trap(&mut self.reg, &mut self.mem) {
            Ok(()) => Ok(()),
            Err(err) => if err.description() == "HALT" {
                Err("HALT")
            } else {
                println!("ERROR: {}", err);
                Err("IO ERROR")
            }
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let pc = self.read(REG::PC);
        let instr = self.mem.read(pc);
        self.write(REG::PC, pc+1);
        instr
    }
}
