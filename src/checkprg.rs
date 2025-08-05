// Unused file

pub fn load_program(filename: &str, load_addr: u16)
{   let mut file = File::open(filename).expect("Failed to open input file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut type = 0;

    let is_ines = buffer.len() >= 16 && &buffer[0..4] == b"NES\x1A"; // The outdated INES format
                                                                     // uses 0x1A, the MS-DOS new
                                                                     // line, that's sad.
    let is_nes2 = is_ines && (buffer[7] & 0x0C) == 0x08;

    if is_nes2
    {   println!("NES2 format... I think...");
    } else if is_ines
    {   println!("INES format... I think...");
    }

/*
    unsafe
    {   for (i, byte) in buffer.iter().enumerate()
        {   MEMORY[load_addr as usize + i] = *byte;
        }
        MEMORY[0xFFFC] = (load_addr & 0xFF) as u8;
        MEMORY[0xFFFD] = (load_addr >> 8) as u8;
    }
*/
}
