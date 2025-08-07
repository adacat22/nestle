Nestle – now with 100% less child labor!
========================================

This is just an NES emulator I'm writing for fun.

See https://github.com/christopherpow/nes-test-roms if you want to test this emulator

========================================

ABOUT THE CPUS:

The NES uses CPUs with MOS 6502 instruction sets, customized by Ricoh.

RICOH 2A03 / RP2A03 (NTSC):  
  * Max. CPU Clock:    ~1.79 MHz  
  * Max. Frame Rate:   ~60.1 Hz (59.94)  
  * Audio:             5-channel APU (2 pulse, 1 triangle, 1 noise, 1 DPCM)  
  * Notes:             Standard in North American and Japanese consoles. No decimal mode.

RICOH 2A07 / RP2A07 (PAL):  
  * Max. CPU Clock:    ~1.66 MHz  
  * Max. Frame Rate:   50 Hz  
  * Audio:             Same 5 channels, but with different timing and frequency tuning  
  * Notes:             Used in European/Australian consoles. Slower timing affects game speed and music pitch.

========================================

TODO:
  1. Implement UNIF (Universal NES Image Format) support
  2. Fix INES files loading from 0x00 (executes the header text)
