mod regs;

fn init(rom: &mut [u16]) {
    rom[0] = regs::ldh(regs::REG0, 0);
    rom[1] = regs::ldl(regs::REG0, 0);
    rom[2] = regs::ldh(regs::REG1, 0);
    rom[3] = regs::ldl(regs::REG1, 1);
    rom[4] = regs::ldh(regs::REG2, 0);
    rom[5] = regs::ldl(regs::REG2, 0);
    rom[6] = regs::ldh(regs::REG3, 0);
    rom[7] = regs::ldl(regs::REG3, 10);
    rom[8] = regs::add(regs::REG2, regs::REG1);
    rom[9] = regs::add(regs::REG0, regs::REG2);
    rom[10] = regs::st(regs::REG0, 64);
    rom[11] = regs::cmp(regs::REG2, regs::REG3);
    rom[12] = regs::je(14);
    rom[13] = regs::jmp(8);
    rom[14] = regs::hlt();
}

fn main() {
    // 初期化
    let mut reg: [u16; 8] = [0; 8];
    let mut rom: [u16; 256] = [0; 256];
    let mut ram: [u16; 256] = [0; 256];

    init(&mut rom);
    
    let mut pc = 0;
    let mut ir;
    let mut flag_eq: bool = false;

    loop {
        // 終了
        ir = rom[pc];
        pc += 1;
        println!("ir:{:b} pc:{}", ir, pc);

        let code = regs::op_code(ir);
        let reg_a = regs::op_reg_a(ir) as usize;
        let reg_b = regs::op_reg_b(ir) as usize;
        let data = regs::op_data(ir);
        let addr = regs::op_addr(ir) as usize;

        match code {
            regs::MOV => reg[reg_a] = reg[reg_b],
            regs::ADD => reg[reg_a] = reg[reg_a] + reg[reg_b],
            regs::SUB => reg[reg_a] = reg[reg_a] - reg[reg_b],
            regs::AND => reg[reg_a] = reg[reg_a] & reg[reg_b],
            regs::OR => reg[reg_a] = reg[reg_a] | reg[reg_b],
            regs::SL => reg[reg_a] = reg[reg_a] << 1,
            regs::SR => reg[reg_a] = reg[reg_a] >> 1,
            regs::SRA => reg[reg_a] = reg[reg_a] & 0x8000 | (reg[reg_a] >> 1),
            regs::LDL => reg[reg_a] = reg[reg_a] & 0x7700 | data,
            regs::LDH => reg[reg_a] = reg[reg_a] & 0x0077 | data >> 8,
            regs::CMP => flag_eq = reg[reg_a] == reg[reg_b],
            regs::JE => if flag_eq {pc = addr},
            regs::JMP => pc = addr,
            regs::LD => reg[reg_a] = ram[addr],
            regs::ST => ram[addr] = reg[reg_a],
            _ => (),
        }
        println!("{}", ram[64]);

        if ir == regs::hlt() {
            break;
        }
    }

    println!("Hello, world!");
}
