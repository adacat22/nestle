mod ricoh2A03;
//mod ricoh2A07;

use std::env;
use std::process::ExitCode;
use ricoh2A03::{CPU, load_program};
//use ricoh2A07::{CPU, load_program};

fn main() -> ExitCode
{   let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {   eprintln!("Usage: {} file.bin", args[0]);
        return ExitCode::SUCCESS;
    }

    let filename = &args[1];
    let load_address = 0x0600;

    load_program(filename, load_address);

    let mut cpu = CPU::new();
    cpu.reset();
    cpu.run();

    ExitCode::SUCCESS
}
