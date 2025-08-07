mod ricoh2A03;
mod checkprg;
//mod ricoh2A07;

use std::env;
use std::process::ExitCode;
//use ricoh2A03::{CPU, load_program};
use crate::ricoh2A03::CPU;
use checkprg::load_prg;
//use ricoh2A07::{CPU, load_program};

fn main() -> ExitCode
{   let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {   eprintln!("Usage: {} cool_game.nes", args[0]);
        return ExitCode::FAILURE;
    }

    let filename = &args[1];
    let load_address = 0x0600;

    load_prg(filename, load_address);

    let mut cpu = CPU::new();
    cpu.reset();
    cpu.run();

    ExitCode::SUCCESS
}
