## CHIP-8 Specification Notes

### CPU:
- 4kb of ram
- addressable from 0x000 -> 0xFFF

- programs start at:
    - 0x200 -> Normal
    - 0x600 -> ETI 660 Chip-8 program start location

- 16 8bit registers (v[0] -> v[15]) (Data Registers)
    - 16th (v[15]) register is used for the carry flag
    - accepted values are 0x00 -> 0xFF

- 16bit 'I' register used to stored memory addresses for later use

- Display: 64x32

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


## Instructions

- 0nnn - SYS addr
  - Jump to machine code routine at nnn
  - Only used on old machines not modern interpreters

- 00e0 - CLS
  - Clear the display

- 00ee - RET
  - Return from subroutine
  - Set program counter to the addr at top of the stack subtract 1 from stack pointer

- 1nnn - JP addr
  - Jump to location at nnn
  - Interpreter sets program counter to nnn

- 2nnn - CALL addr
  - Call subroutine at nnn
  - Increments stack pointer, then puts current PC on top of the stack, PC then set to nnn

- 3xkk - SE Vx, byte
  - Skip next instruction if Vx = kk
  - Compares register Vx to kk, and if they are equal, increments the program counter by 2

- 4xkk - SNE Vx, byte
  - Skip next instruction if Vx != nn
  - Compare register Vx to kk, if not equal, increment program counter by 2

- 5xy0 - SE Vx, Vy
  - Skip next instruction if Vx = Vy
  - Compare Vx to Vy, if they are equal, increment program counter by 2

- 6xkk - LD Vx, byte
  - Set Vx = nn
  - Put value nn into register Vx

- 7xkk - ADD Vx, byte
  - Set Vx = Vx + nn
  - Add the value nn to value of register Vx, then store in Vx

- 8xy0 - LD Vx, Vy
  - Set Vx = Vy
  - Store the value of Vy in Vx

- 8xy1 - OR Vx, Vy
  - Set Vx = Vx OR Vy
  - Preform a bitwise OR on the values Vx and Vy, store result in Vx. 

- 8xy2 - AND Vx, Vy
  - Set Vx = Vx AND Vy
  - Preform a bitwise AND on the values Vx and Vy, store result in Vx

- 8xy3 - XOR Vx, Vy
  - Set Vx = Vx XOR Vx
  - Perform a bitwise XOR on the valyes Vx and Vy, store the result in Vx

- 8xy4 - ADD Vx, Vy
  - Set Vx = Vx + Vy, set VF = carry
  - The values of Vx and Vy are added together. If the result is greater than 8bits (255), set VF = 1 otherwise 0. Only lowest 8 bits of result are kept and stored in 

- 8xy5 - SUB Vx, Vy
  - Set Vx = Vx - Vy, set VF = NOT borrow
  - If Vx > Vy, set VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results are stored in Vx

- 8xy6 - SHR Vx, {, Vy}
  - Set Vx = Vx SHR 1
  - If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2

- 8xy7 - SUBN Vx, Vy
  - Set Vx = Vy - Vx, set VF = NOT borrow
  - If Vy > Vx then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, results are stored in Vx

- 8xyE - SHL Vx {, Vy}
  - Set Vx = Vx SHL 1
  - If the most-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is multiplied by 2

- 9xy0 - SNE Vx, Vy
  - Skip next instruction if Vx != Vy
  - The value of Vx and Vy are compared, if they are not equal, increase the program counter by 2

- Annn - LD I, addr
  - Set I = nnn
  - The value of register I is set to nnn

- Bnnn - JP V0, addr
  - Jump to location nnn + V0
  - Set the program counter to (nnn + value of register V0)

- Cxkk - RND Vx, byte
  - Set Vx = random byte AND nn
  - Generate a random number from 0 to 255 which is then ANDed with nn. 

- Dxyn - DRW Vx, Vy, nibble
  - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
  - Read n bytes from ram, starting at addr stored in I. These bytes are then disiplayed as sprites on screen at coord (Vx, Vy). Sprites are XORed onto the screen, if this causes pixels to be erased, set VF to 1 otherwise, set to 0. If the sprite is positioned so part of it is outside the coordinates of the screen, wrap it to the opposite side of the screen.

- Ex9E - SKP Vx
  - Skip next instruction if the key with the value of Vx is pressed
  - Check keyboard, if key corresponding to Vx is currently down, increase program counter by 2

- ExA1 - SKNP Vx
  - Skip next instruction if key with the value of Vx is not pressed
  - Check keyboard, if key corresponding to Vx is is currently up, increase program counter by 2

- Fx07 0 LD Vx, DT
  - Set Vx = delay timer value
  - Place the value of the delay timer into Vx

- Fx0A - LD Vx, K
  - Wait for a key press, store the value of the key in Vx
  - All execution stops until a key is pressed, then the value of that key is stored in Vx

- Fx15 - LD DT, Vx
  - Set delay timer = Vx
  - Delay timer is set to the value of Vx register

- Fx18 - LD ST, Vx
  - Set sound timer = Vx
  - Sound timer is set to the value in the Vx register

- Fx1E - ADD I, Vx
  - Set I = I + Vx
  - The value of I and Vx register are added, results stored in I

- Fx29 - LD F, Vx
  - Set I = location of sprite for digit Vx
  - The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx

- Fx33 - LD B, Vx
  - Store BCD representation of Vx in memory locations I, I+1, I+2
  - Take decimal value of Vx and place hundredss digit in memory at location in I, I+1, I+2

- Fx55 - LD [I], Vx
  - Store registers V0 through Vx in memory starting at location I
  - Copy the values of registers V0 through Vx into memory, starting at addr in I

- Fx65 - LD Vx, [I]
  - Read registers V0 through Vx from memory starting at location I
  - Read values from memory starting at location I into registers V0 through Vx