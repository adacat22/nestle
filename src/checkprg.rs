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
