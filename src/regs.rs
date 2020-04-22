// 命令コードのインデックス
pub const MOV:u16 = 0;
pub const ADD:u16 = 1;
pub const SUB:u16 = 2;
pub const AND:u16 = 3;
pub const OR:u16 = 4;
pub const SL:u16 = 5;
pub const SR:u16 = 6;
pub const SRA:u16 = 7;
pub const LDL:u16 = 8;
pub const LDH:u16 = 9;
pub const CMP:u16 = 10;
pub const JE:u16 = 11;
pub const JMP:u16 = 12;
pub const LD:u16 = 13;
pub const ST:u16 = 14;
pub const HLT:u16 = 15;

// レジスタのインデックス
pub const REG0:u16 = 0;
pub const REG1:u16 = 1;
pub const REG2:u16 = 2;
pub const REG3:u16 = 3;
pub const REG4:u16 = 4;
pub const REG5:u16 = 5;
pub const REG6:u16 = 6;
pub const REG7:u16 = 7;

// 機械語に変換
pub fn mov(ra: u16, rb: u16) -> u16 {
    (MOV << 11) | (ra << 8) | (rb << 5)
}
pub fn add(ra: u16, rb: u16) -> u16 {
    (ADD << 11) | (ra << 8) | (rb << 5)
}
pub fn sub(ra: u16, rb: u16) -> u16 {
    (SUB << 11) | (ra << 8) | (rb << 5)
}
pub fn and(ra: u16, rb: u16) -> u16 {
    (AND << 11) | (ra << 8) | (rb << 5)
}
pub fn or(ra: u16, rb: u16) -> u16 {
    (OR << 11) | (ra << 8) | (rb << 5)
}
pub fn sl(ra: u16) -> u16 {
    (SL << 11) | (ra << 8)
}
pub fn sr(ra: u16) -> u16 {
    (SR << 11) | (ra << 8)
}
pub fn sra(ra: u16) -> u16 {
    (SRA << 11) | (ra << 8)
}
pub fn ldl(ra: u16, ival: u16) -> u16 {
    (LDL << 11) | (ra << 8) | (ival & 0x00ff)
}
pub fn ldh(ra: u16, ival: u16) -> u16 {
    (LDH << 11) | (ra << 8) | (ival & 0x00ff)
}
pub fn cmp(ra: u16, rb: u16) -> u16 {
    (CMP << 11) | (ra << 8) | (rb << 5)
}
pub fn je(addr: u16) -> u16 {
    (JE << 11) | (addr & 0x00ff)
}
pub fn jmp(addr: u16) -> u16 {
    (JMP << 11) | (addr & 0x00ff)
}
pub fn ld(ra: u16, addr: u16) -> u16 {
    (LD << 11) | (ra << 8) | (addr & 0x00ff)
}
pub fn st(ra: u16, addr: u16) -> u16 {
    (ST << 11) | (ra << 8) | (addr & 0x00ff)
}
pub fn hlt() -> u16 {
    HLT << 11
}

// 操作
pub fn op_code(ir: u16) -> u16 {
    ir >> 11
}
pub fn op_reg_a(ir: u16) -> u16 {
    (ir >> 8) & 0x0007
}
pub fn op_reg_b(ir: u16) -> u16 {
    (ir >> 5) & 0x0007
}
pub fn op_data(ir: u16) -> u16 {
    ir & 0x00ff
}
pub fn op_addr(ir: u16) -> u16 {
    ir & 0x00ff
}