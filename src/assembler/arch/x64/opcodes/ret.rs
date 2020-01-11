use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::X64Instruction;

impl X64Assembler {
    pub fn generate_ret_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        codes.push(0xc3);
    }
}
