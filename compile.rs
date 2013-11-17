mod parse;

// instruction opcodes
//
// more opcodes are probably required, but for now:
//
// Literal      = check if the input matches a literal
// Range        = check if the input is within a range
// Match        = the input matches!
// Jump         = goto a point in the stack
// CaptureStart = start capturing the input
// CaptureEnd   = end capturing the input
// Split        = start a new thread with one jumping to the
//                first uint, and the next jumping to the
//                second.

enum OpCode {
  Literal(~str),
  Range(char, char),
  Match,
  Jump(uint),
  CaptureStart,
  CaptureEnd,
  Split(uint, uint)
}

struct Instruction {
  opcode: OpCode
}

fn compile(ps: parse::Regexp) -> ~[Instruction] {

}
