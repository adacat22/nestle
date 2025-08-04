mod ricoh;

use std::env;
use ricoh::{CPU, load_program};

fn main()
{   println!("Nestle emulator - with 100% less child labor!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {   eprintln!("Usage: {} file.bin", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let load_address = 0x0600;

    load_program(filename, load_address);

    let mut cpu = CPU::new();
    cpu.reset();
    cpu.run();
}
