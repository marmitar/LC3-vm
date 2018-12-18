mod lib;
use lib::memory::Memory;
use lib::cpu::CPU;

extern crate nix;
use nix::sys::termios::{tcgetattr, tcsetattr, LocalFlags, SetArg};
use std::os::unix::io::AsRawFd;

fn main() {
    let stdin_fd = std::io::stdin().as_raw_fd();
    let old_tio = match tcgetattr(stdin_fd) {
        Err(_) => return,
        Ok(tio) => tio
    };
    {
        let mut new_tio = old_tio.clone();
        let flags = new_tio.local_flags & !LocalFlags::ICANON & !LocalFlags::ECHO;
        new_tio.local_flags = flags;
        if tcsetattr(stdin_fd, SetArg::TCSANOW, &new_tio).is_err() {
            panic!("Error: terminal misconfigured");
        }
    }

    for program in std::env::args().skip(1) {
        println!("Running {}", &program);

        let mut mem = match Memory::load_program(&program) {
            Ok(mem) => mem,
            _ => return println!("failed to load image: {}", &program)
        };
        let mut cpu = CPU::new(&mut mem);

        loop {
            let instr = cpu.fetch();
            if cpu.run(instr).is_err() {
                break
            };
        }
    }

    if tcsetattr(stdin_fd, SetArg::TCSANOW, &old_tio).is_ok() {
        println!("");
    } else {
        panic!("Error: terminal misconfigured");
    }
}
