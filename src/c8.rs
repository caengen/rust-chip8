struct Chip8 {
  opcode: u16,
  memory: [u8; 4096],
  rom: [u8; 4096 - 516],
  // registers V0...VE
  V: [u8; 16],
  // index register
  I: u16,
  // program counter
  pc: u16,
  // pixel graphics. 64x34, 2048 pixels total
  gfx: [u8; 64 * 32],
  // timer registers
  // 60 Hz timer registers
  delayTimer: u8,
  soundTimer: u8,
  // stack
  stack: [u16; 16],
  sp: u8,

  // key state
  keys: [u8; 16],
  debug: bool
}

impl Chip8 {
  fn initialize() {

  }

}