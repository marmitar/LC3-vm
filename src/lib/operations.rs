use lib::reference::{REG, OP};
use lib::cpu::CPU;

fn sign_extend(value: u16, bits: u8) -> u16 {
    let sign = (value >> (bits-1)) & 1;
    let mask = 0xFFFF << bits;

    if sign == 1 {
        value | mask
    } else {
        value & !mask
    }
}

impl <'a> CPU<'a> {
    fn and_add(&mut self, instr: u16, and: bool) -> Result<(), &'static str> {
        let imm = (instr >> 5) & 0b1 > 0;

        let dr = (instr >> 9) & 0b111;
        let sr1 = (instr >> 6) & 0b111;

        let first = self.read(REG::new(sr1)?);
        let other = if imm {
            sign_extend(instr, 5)
        } else {
            let reg = instr & 0b111;
            self.read(REG::new(reg)?)
        };

        let result = if and {first & other} else {first + other};
        self.write(REG::new(dr)?, result);
        self.update_flags(REG::new(dr)?);

        Ok(())
    }

    fn and(&mut self, instr: u16) -> Result<(), &'static str> {
        self.and_add(instr, true)
    }

    fn add(&mut self, instr: u16) -> Result<(), &'static str> {
        self.and_add(instr, false)
    }

    fn not(&mut self, instr: u16) -> Result<(), &'static str> {
        let sr = (instr >> 6) & 0b111;
        let dr = (instr >> 9) & 0b111;

        let src = REG::new(sr)?;
        let dest = REG::new(dr)?;

        let value = self.read(src);
        self.write(dest, !value);
        self.update_flags(dest);

        Ok(())
    }

    fn effective_address(&self, instr: u16, bits: u8) -> u16 {
        let offset = sign_extend(instr, bits);
        let pc = self.read(REG::PC);
        pc + offset
    }

    fn base_jump(&mut self, offset: bool, value: u16, bits: u8) -> Result<(), &'static str> {
        let address = if offset {
            self.effective_address(value, bits)
        } else {
            let reg = REG::new(value)?;
            self.read(reg)
        };

        self.write(REG::PC, address);
        Ok(())
    }

    fn branch(&mut self, instr: u16) -> Result<(), &'static str> {
        let nzp = (instr >> 9) & 0b111;
        let flags = self.read(REG::COND);

        if (nzp & flags) != 0 {
            self.base_jump(true, instr, 9)?
        }

        Ok(())
    }

    fn jump(&mut self, instr: u16) -> Result<(), &'static str> {
        let reg = (instr >> 6) & 0b111;
        self.base_jump(false, reg, 0)
    }

    fn jump_to_subroutine(&mut self, instr: u16) -> Result<(), &'static str> {
        let pc = self.read(REG::PC);
        self.write(REG::R7, pc);

        let reg_mode = ((instr >> 11) & 1) == 0;
        let value = if reg_mode {
            (instr >> 6) & 0b111
        } else {
            instr
        };

        self.base_jump(!reg_mode, value, 11)
    }

    fn base_load(&mut self, dr: u16, address: u16) -> Result<(), &'static str> {
        let dest = REG::new(dr)?;
        let value = self.mem.read(address);

        self.write(dest, value);
        self.update_flags(dest);
        Ok(())
    }

    fn load(&mut self, instr: u16) -> Result<(), &'static str> {
        let address = self.effective_address(instr, 9);
        let dr = (instr >> 9) & 0b111;
        self.base_load(dr, address)
    }

    fn load_register(&mut self, instr: u16) -> Result<(), &'static str> {
        let reg = (instr >> 6) & 0b111;
        let base = self.read(REG::new(reg)?);
        let offset = sign_extend(instr, 6);

        let dr = (instr >> 9) & 0b111;
        self.base_load(dr, base + offset)
    }

    fn load_indirect(&mut self, instr: u16) -> Result<(), &'static str> {
        let base = self.effective_address(instr, 9);
        let address = self.mem.read(base);

        let dr = (instr >> 9) & 0b111;
        self.base_load(dr, address)
    }

    fn load_effective_address(&mut self, instr: u16) -> Result<(), &'static str> {
        let reg = REG::new((instr >> 9) & 0b111)?;
        let eff_adr = self.effective_address(instr, 9);

        self.write(reg, eff_adr);
        self.update_flags(reg);
        Ok(())
    }

    fn base_store(&mut self, sr: u16, address: u16) -> Result<(), &'static str> {
        let src = REG::new(sr)?;
        let value = self.read(src);

        self.mem.write(address, value);
        Ok(())
    }

    fn store(&mut self, instr: u16) -> Result<(), &'static str> {
        let address = self.effective_address(instr, 9);
        let sr = (instr >> 9) & 0b111;
        self.base_store(sr, address)
    }

    fn store_register(&mut self, instr: u16) -> Result<(), &'static str> {
        let reg = (instr >> 6) & 0b111;
        let base = self.read(REG::new(reg)?);
        let offset = sign_extend(instr, 6);

        let sr = (instr >> 9) & 0b111;
        self.base_store(sr, base + offset)
    }

    fn store_indirect(&mut self, instr: u16) -> Result<(), &'static str> {
        let base = self.effective_address(instr, 9);
        let address = self.mem.read(base);

        let sr = (instr >> 9) & 0b111;
        self.base_store(sr, address)
    }

    pub fn run(&mut self, instr: u16) -> Result<(), &'static str> {
        let op = OP::new(instr >> 12)?;

        match op {
            OP::BR => self.branch(instr),
            OP::ADD => self.add(instr),
            OP::LD => self.load(instr),
            OP::ST => self.store(instr),
            OP::JSR => self.jump_to_subroutine(instr),
            OP::AND => self.and(instr),
            OP::LDR => self.load_register(instr),
            OP::STR => self.store_register(instr),
            OP::RTI => Err("not implemented"),
            OP::NOT => self.not(instr),
            OP::LDI => self.load_indirect(instr),
            OP::STI => self.store_indirect(instr),
            OP::JMP => self.jump(instr),
            OP::RES => Err("reserved"),
            OP::LEA => self.load_effective_address(instr),
            OP::TRAP => self.run_trap(instr as u8)
        }
    }
}
