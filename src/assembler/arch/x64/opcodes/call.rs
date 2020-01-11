use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{X64InstName, X64Instruction, X64Operand};

impl X64Assembler {
    pub fn generate_callrm64_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        // call-opcode
        codes.push(0xff);

        // call - register
        codes.push(0xd0);
    }
}

impl X64Instruction {
    pub fn change_call_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // call r/m64
            _ => X64InstName::CALLRM64,
        }
    }
}
