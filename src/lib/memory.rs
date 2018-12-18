extern crate itertools;
extern crate nix;

use lib::memory::itertools::Itertools;
use std::fs::File;
use std::io::Read;

use lib::memory::nix::sys::select::{FdSet, select};
use lib::memory::nix::sys::time::{TimeValLike, TimeVal};
use std::os::unix::io::AsRawFd;

fn agg_ne_bytes(bytes: Vec<u8>) -> u16 {
    if cfg!(target_endian = "big") {
        ((bytes[1] as u16) << 8) | (bytes[0] as u16)
    } else {
        ((bytes[0] as u16) << 8) | (bytes[1] as u16)
    }
}


pub const MEM_SIZE: usize = std::u16::MAX as usize;

pub struct Memory {
    space: [u16; MEM_SIZE]
}

pub const MR_KBSR: u16 = 0xFE00;  // keyboard status
pub const MR_KBDR: u16 = 0xFE02;  // keyboard data

fn kb_buffered() -> bool {
    let mut readfds = FdSet::new();
    readfds.clear();

    let stdin_fd = std::io::stdin().as_raw_fd();
    readfds.insert(stdin_fd);

    let mut timeout = TimeVal::zero();
    match select(Some(1), Some(&mut readfds), None, None, Some(&mut timeout)) {
        Ok(0) => false,
        _ => true
    }
}

impl Memory {
    pub fn read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            let mut buffer: [u8; 1] = [0];
            let stdin = std::io::stdin();

            if kb_buffered() && stdin.lock().read_exact(&mut buffer).is_ok() {
                self.space[MR_KBSR as usize] = 0xFFFF;
                self.space[MR_KBDR as usize] = buffer[0] as u16;
            } else {
                self.space[MR_KBSR as usize] = 0;
            }
        }

        self.space[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.space[address as usize] = value;
    }


    pub fn load_program(filename: &String) -> std::io::Result<(Memory)> {
        let mut file = File::open(filename)?;

        let mut address: u16 = {
            let mut origin_buf = vec![0; 2];
            file.read_exact(&mut origin_buf)?;
            agg_ne_bytes(origin_buf)
        };

        let mut bytes = Vec::<u8>::new();
        file.read_to_end(&mut bytes)?;

        let mut mem = Memory {space: [0; MEM_SIZE]};
        for word in &bytes.into_iter().chunks(2) {
            let halfwords = word.collect_vec();
            let value = agg_ne_bytes(halfwords);
            mem.write(address, value);

            address += 1
        }

        Ok(mem)
    }
}
