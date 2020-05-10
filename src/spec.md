## CHIP-8 Specification Notes

### CPU:
- 4kb of ram
- addressable from 0x000 -> 0xFFF

- programs start at:
    - 0x200 -> Normal
    - 0x600 -> ETI 660 Chip-8 program start location

- 16 8bit registers (v[0] -> v[15]) (Data Registers)
    - 16th register is used for the carry flag
    - accepted values are 0x00 -> 0xFF

- 16bit 'I' register used to stored memory addresses for later use

- 8bit delay timer
- 8bit sound timer
    - When these registers are non-zero, decrement them at the rate of 60hz till they reach 0

- Keyboard was a 16-key hex keypad

- Chip8 System has 2kb of vram to represent a 64x32 screen of pixels

- Chip8 draws graphics through the use of sprites (groups of pixel bytes)
    - Each sprites pixel bytes are in binary representation of the graphic it is displaying
    - Sprites are comprised of 15 bytes of graphics making the max size of 8x15 pixels 

- Each instruction is 2bytes wide with the data encoded directly in the instruction
    - Instructions will be represented by an array of function pointers to their respective opcode
    - Need to determine how to translate 2byte instruction into an index representing the opcode to call