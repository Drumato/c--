use crate::assembler::arch::x64::inst::X64Instruction;
use crate::assembler::arch::x64::X64Assembler;

impl X64Assembler {
    pub fn generate_ret_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        codes.push(0xc3);
    }
}
