struct Chip8 {
  opcode: u16;
  memory: u8[];
  rom: u8[];
  // registers V0...VE
  V: u8[];
  // index register
  I: u16;
  // program counter
  pc: u16;
  // pixel graphics. 64x34, 2048 pixels total
  gfx: u8[];
  // timer registers
  // 60 Hz timer registers
  delayTimer: u16;
  soundTimer: u16;
  // stack
  stack: u16[];
  sp: u8;

  // key state
  keys: u8[];
  debug: bool;
}