use crate::assembler::arch::x64::assembler::X64Assembler;

impl X64Assembler {
    pub fn generate_syscall_inst(codes: &mut Vec<u8>) {
        // syscall-opcode
        codes.push(0x0f);
        codes.push(0x05);
    }
}
