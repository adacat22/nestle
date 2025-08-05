/*
 * This is the CPU the NTSC version uses
 *
 * "what's the difference?"
 *
 * See readme.txt
 */

use std::fs::File;
use std::io::Read;

const FLAG_C: u8 = 0x01;
const FLAG_Z: u8 = 0x02;
const FLAG_I: u8 = 0x04;
const FLAG_D: u8 = 0x08;
const FLAG_B: u8 = 0x10;
const FLAG_U: u8 = 0x20;
const FLAG_V: u8 = 0x40;
const FLAG_N: u8 = 0x80;

pub static mut MEMORY: [u8; 0x10000] = [0; 0x10000];

pub struct CPU
{   pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
    pub running: bool,
    pub cycles: u64,

    pub use_illegal_opcodes: bool,  // The usage of unofficial opcodes is rare in NES games, if
                                    // (for some reason), you want to disable them, you can.
                                    //
                                    // Unused for now...
                                    //
                                    // DEFAULT: true

    pub use_wdc_extentions: bool,   // Just like the the boolean above, you will never need this,
                                    // the NES was not designed to be used with a WDC (Western
                                    // Design Center) chip.
                                    //
                                    // Unused for now...
                                    //
                                    // DEFAULT: false
}

impl CPU
{   pub fn new() -> Self
    {   CPU
        {   a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFD,
            p: 0x34,
            running: true,
            cycles: 0,
            use_illegal_opcodes: true,
            use_wdc_extentions: false,
        }
    }

    pub fn reset(&mut self)
    {   unsafe
        {   self.pc = (MEMORY[0xFFFC] as u16) | ((MEMORY[0xFFFD] as u16) << 8);
        }
    }

    fn set_zn_flags(&mut self, value: u8)
    {   self.p &= !(FLAG_Z | FLAG_N);
        if value == 0
        {   self.p |= FLAG_Z;
        }
        if value & 0x80 != 0
        {   self.p |= FLAG_N;
        }
    }

    fn ldx_imm(&mut self)
    {   unsafe
        {   self.x = MEMORY[self.pc as usize];
        }
        self.pc += 1;
        self.set_zn_flags(self.x);
    }

    fn ldy_imm(&mut self)
    {   unsafe
        {   self.y = MEMORY[self.pc as usize];
        }
        self.pc += 1;
        self.set_zn_flags(self.y);
    }

    fn sty_absx(&mut self)
    {   unsafe
        {   let lo = MEMORY[self.pc as usize] as u16;
            let hi = MEMORY[(self.pc + 1) as usize] as u16;
            let mut addr = (hi << 8) | lo;
            addr = addr.wrapping_add(self.x as u16);
            MEMORY[addr as usize] = self.y;
        }
        self.pc += 2;
    }

    fn inx(&mut self)
    {   self.x = self.x.wrapping_add(1);
        self.set_zn_flags(self.x);
    }

    fn iny(&mut self)
    {   self.y = self.y.wrapping_add(1);
        self.set_zn_flags(self.y);
    }

    fn cpy_imm(&mut self)
    {   unsafe
        {   let value = MEMORY[self.pc as usize];
            self.pc += 1;
            let result = self.y.wrapping_sub(value);
            self.p &= !(FLAG_C | FLAG_Z | FLAG_N);
            if self.y >= value
            {   self.p |= FLAG_C;
            }
            if result == 0
            {   self.p |= FLAG_Z;
            }
            if result & 0x80 != 0
            {   self.p |= FLAG_N;
            }
        }
    }

    fn bne(&mut self)
    {   unsafe 
        {   let offset = MEMORY[self.pc as usize] as i8;
            self.pc += 1;
            if (self.p & FLAG_Z) == 0
            {   self.pc = (self.pc as i32 + offset as i32) as u16;
            }
        }
    }

    fn brk(&mut self) // 0x00
    {   self.running = false;
    }

    pub fn exec_opcode(&mut self, opcode: u8)
    {   match opcode
        {   0xA2 => self.ldx_imm(),
            0xA0 => self.ldy_imm(),
            0x99 => self.sty_absx(),
            0xE8 => self.inx(),
            0xC8 => self.iny(),
            0xC0 => self.cpy_imm(),
            0xD0 => self.bne(),
            0x00 => self.brk(),
            _ =>
            {   println!("ILLEGAL OPCODE!!! {:02X}", opcode);
                self.running = false;
            }
        }
    }

    pub fn run(&mut self)
    {   while self.running
        {   unsafe
            {   let opcode = MEMORY[self.pc as usize];
                println!(
                    "OPCODE={:02X} PC={:04X} A={} X={} Y={} P={:02X}",
                    opcode, self.pc, self.a, self.x, self.y, self.p
                );
                self.pc += 1;
                self.exec_opcode(opcode);
            }
        }
    }
}

pub fn load_program(filename: &str, load_addr: u16)
{   let mut file = File::open(filename).expect("Failed to open input file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    unsafe
    {   for (i, byte) in buffer.iter().enumerate()
        {   MEMORY[load_addr as usize + i] = *byte;
        }
        MEMORY[0xFFFC] = (load_addr & 0xFF) as u8;
        MEMORY[0xFFFD] = (load_addr >> 8) as u8;
    }
}
