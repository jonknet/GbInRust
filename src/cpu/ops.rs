use crate::cpu::*;

// Base functions

impl Cpu {
    pub fn executeop(&mut self) {
        let opcode = self.M.read(self.R.pc);
        let arg1 = self.M.read(self.R.pc + 1);
        let arg1_i = self.M.read(self.R.pc + 1) as i8;
        let arg2 = self.M.read(self.R.pc + 2);
        let d16 = ((arg2 as u16) << 8) | arg1 as u16;
        println!("pc {:x} : op {:x} ({:x} {:x}) : f {:x} a {:x} b {:x} c {:x} d {:x} e {:x} h {:x} l {:x} : sp {:x} ({:x} {:x})",
                 self.R.pc,opcode,arg1,arg2,self.R.f,self.R.a,self.R.b,self.R.c,self.R.d,self.R.e,self.R.h,self.R.l,self.R.sp,self.M.read(self.R.sp),self.M.read(self.R.sp+1));
        if opcode != 0xCB {
            self.R.pc += OP_LEN[opcode as usize] as u16;
        }
        self.T(4); // Since all ops take at least 4 cycles...
        match opcode {
            0x0 => {}
            0x1 => self.R.set_bc(d16),
            0x2 => self.M.write(self.R.get_bc(), self.R.a),
            0x3 => self.R.set_bc(self.R.get_bc() + 1),
            0x4 => {
                self.inc(Register::b);
            }
            0x5 => {
                self.dec(Register::b);
            }
            0x6 => self.R.b = arg1,
            0x7 => self.rla(true),
            0x8 => self.M.write16(d16, self.R.get_sp()),
            0x9 => self.add16(Register::hl, Register::bc),
            0xA => self.R.a = self.M.read(self.R.get_bc()),
            0xB => self.R.set_bc(self.R.get_bc() - 1),
            0xC => self.inc(Register::c),
            0xD => self.dec(Register::c),
            0xE => self.R.c = arg1,
            0xF => self.rra(true),
            0x10 => {
                self.S.halted = true;
            }
            0x11 => self.R.set_de(d16),
            0x12 => self.M.write(self.R.get_de(), self.R.a),
            0x13 => self.R.set_de(self.R.get_de() + 1),
            0x14 => {
                self.inc(Register::d);
            }
            0x15 => {
                self.dec(Register::d);
            }
            0x16 => self.R.d = arg1,
            0x17 => self.rla(false),
            0x18 => {
                self.jr(true, arg1_i);
            }
            0x19 => self.add16(Register::hl, Register::de),
            0x1A => self.R.a = self.M.read(self.R.get_de()),
            0x1B => self.R.set_de(self.R.get_de() - 1),
            0x1C => self.inc(Register::e),
            0x1D => self.dec(Register::e),
            0x1E => self.R.e = arg1,
            0x1F => self.rra(false),
            0x20 => self.jr(self.get(Z) == 0, arg1_i),
            0x21 => self.R.set_hl(d16),
            0x22 => {
                self.M.write(self.R.get_hl(), self.R.a);
                self.R.set_hl(self.R.get_hl() + 1);
            }
            0x23 => self.R.set_hl(self.R.get_hl() + 1),
            0x24 => {
                self.inc(Register::h);
            }
            0x25 => self.dec(Register::h),
            0x26 => self.R.h = arg1,
            0x27 => self.daa(),
            0x28 => self.jr(self.get(Z) != 0, arg1_i),
            0x29 => self.add16(Register::hl, Register::hl),
            0x2A => {
                self.R.a = self.M.read(self.R.get_hl());
                self.R.set_hl(self.R.get_hl() + 1);
            }
            0x2B => self.R.set_hl(self.R.get_hl() - 1),
            0x2C => self.inc(Register::l),
            0x2D => self.dec(Register::l),
            0x2E => self.R.l = arg1,
            0x2F => {
                self.R.a = !self.R.a;
                self.set(N);
                self.set(H);
            }
            0x30 => {
                self.jr(self.get(C) == 0, arg1_i);
            }
            0x31 => self.R.set_sp(d16),
            0x32 => {
                self.M.write(self.R.get_hl(), self.R.a);
                self.R.set_hl(self.R.get_hl() - 1);
            }
            0x33 => self.R.set_sp(self.R.get_sp() + 1),
            0x34 => self.inc(Register::hl),
            0x35 => self.dec(Register::hl),
            0x36 => self.M.write(self.R.get_hl(), arg1),
            0x37 => {
                self.set(C);
                self.clr(N);
                self.clr(H);
            }
            0x38 => {
                self.jr(self.get(C) != 0, arg1_i);
            }
            0x39 => self.add16(Register::hl, Register::sp),
            0x3A => {
                self.R.a = self.M.read(self.R.get_hl());
                self.R.set_hl(self.R.get_hl() - 1)
            }
            0x3B => self.R.set_sp(self.R.get_sp() - 1),
            0x3C => self.inc(Register::a),
            0x3D => self.dec(Register::a),
            0x3E => self.R.a = arg1,
            0x3F => {
                if self.get(C) != 0 {
                    self.set(C)
                } else {
                    self.clr(C)
                }
                self.clr(N);
                self.clr(H);
            }
            0x40 => {}
            0x41 => self.R.b = self.R.c,
            0x42 => self.R.b = self.R.d,
            0x43 => self.R.b = self.R.e,
            0x44 => self.R.b = self.R.h,
            0x45 => self.R.b = self.R.l,
            0x46 => self.R.b = self.M.read(self.R.get_hl()),
            0x47 => self.R.b = self.R.a,
            0x48 => self.R.c = self.R.b,
            0x49 => self.R.c = self.R.c,
            0x4A => self.R.c = self.R.d,
            0x4B => self.R.c = self.R.e,
            0x4C => self.R.c = self.R.h,
            0x4D => self.R.c = self.R.l,
            0x4E => self.R.c = self.M.read(self.R.get_hl()),
            0x4F => self.R.c = self.R.a,
            0x50 => self.R.d = self.R.b,
            0x51 => self.R.d = self.R.c,
            0x52 => self.R.d = self.R.d,
            0x53 => self.R.d = self.R.e,
            0x54 => self.R.d = self.R.h,
            0x55 => self.R.d = self.R.l,
            0x56 => self.R.d = self.M.read(self.R.get_hl()),
            0x57 => self.R.d = self.R.a,
            0x58 => self.R.e = self.R.b,
            0x59 => self.R.e = self.R.c,
            0x5A => self.R.e = self.R.d,
            0x5B => self.R.e = self.R.e,
            0x5C => self.R.e = self.R.h,
            0x5D => self.R.e = self.R.l,
            0x5E => self.R.e = self.M.read(self.R.get_hl()),
            0x5F => self.R.e = self.R.a,
            0x60 => self.R.h = self.R.b,
            0x61 => self.R.h = self.R.c,
            0x62 => self.R.h = self.R.d,
            0x63 => self.R.h = self.R.e,
            0x64 => self.R.h = self.R.h,
            0x65 => self.R.h = self.R.l,
            0x66 => self.R.h = self.M.read(self.R.get_hl()),
            0x67 => self.R.h = self.R.a,
            0x68 => self.R.l = self.R.b,
            0x69 => self.R.l = self.R.c,
            0x6A => self.R.l = self.R.d,
            0x6B => self.R.l = self.R.e,
            0x6C => self.R.l = self.R.h,
            0x6D => self.R.l = self.R.l,
            0x6E => self.R.l = self.M.read(self.R.get_hl()),
            0x6F => self.R.l = self.R.a,
            0x70 => self.M.write(self.R.get_hl(), self.R.b),
            0x71 => self.M.write(self.R.get_hl(), self.R.c),
            0x72 => self.M.write(self.R.get_hl(), self.R.d),
            0x73 => self.M.write(self.R.get_hl(), self.R.e),
            0x74 => self.M.write(self.R.get_hl(), self.R.h),
            0x75 => self.M.write(self.R.get_hl(), self.R.l),
            0x76 => self.S.halted = true,
            0x77 => self.M.write(self.R.get_hl(), self.R.a),
            0x78 => self.R.a = self.R.b,
            0x79 => self.R.a = self.R.c,
            0x7A => self.R.a = self.R.d,
            0x7B => self.R.a = self.R.e,
            0x7C => self.R.a = self.R.h,
            0x7D => self.R.a = self.R.l,
            0x7E => self.R.a = self.M.read(self.R.get_hl()),
            0x7F => self.R.a = self.R.a,
            0x80 => self.add(self.R.b),
            0x81 => self.add(self.R.c),
            0x82 => self.add(self.R.d),
            0x83 => self.add(self.R.e),
            0x84 => self.add(self.R.h),
            0x85 => self.add(self.R.l),
            0x86 => self.add(self.M.read(self.R.get_hl())),
            0x87 => self.adc(self.R.a),
            0x88 => self.adc(self.R.b),
            0x89 => self.adc(self.R.c),
            0x8A => self.adc(self.R.d),
            0x8B => self.adc(self.R.e),
            0x8C => self.adc(self.R.h),
            0x8D => self.adc(self.R.l),
            0x8E => self.adc(self.M.read(self.R.get_hl())),
            0x8F => self.adc(self.R.a),
            0x90 => self.sub(self.R.b),
            0x91 => self.sub(self.R.c),
            0x92 => self.sub(self.R.d),
            0x93 => self.sub(self.R.e),
            0x94 => self.sub(self.R.h),
            0x95 => self.sub(self.R.l),
            0x96 => self.sub(self.M.read(self.R.get_hl())),
            0x97 => self.sub(self.R.a),
            0x98 => self.sbc(self.R.b),
            0x99 => self.sbc(self.R.c),
            0x9A => self.sbc(self.R.d),
            0x9B => self.sbc(self.R.e),
            0x9C => self.sbc(self.R.h),
            0x9D => self.sbc(self.R.l),
            0x9E => self.sbc(self.M.read(self.R.get_hl())),
            0x9F => self.sbc(self.R.a),
            0xA0 => self.and(self.R.b),
            0xA1 => self.and(self.R.c),
            0xA2 => self.and(self.R.d),
            0xA3 => self.and(self.R.e),
            0xA4 => self.and(self.R.h),
            0xA5 => self.and(self.R.l),
            0xA6 => self.and(self.M.read(self.R.get_hl())),
            0xA7 => self.and(self.R.a),
            0xA8 => self.xor(self.R.b),
            0xA9 => self.xor(self.R.c),
            0xAA => self.xor(self.R.d),
            0xAB => self.xor(self.R.e),
            0xAC => self.xor(self.R.h),
            0xAD => self.xor(self.R.l),
            0xAE => self.xor(self.M.read(self.R.get_hl())),
            0xAF => self.xor(self.R.a),
            0xB0 => self.or(self.R.b),
            0xB1 => self.or(self.R.c),
            0xB2 => self.or(self.R.d),
            0xB3 => self.or(self.R.e),
            0xB4 => self.or(self.R.h),
            0xB5 => self.or(self.R.l),
            0xB6 => self.or(self.M.read(self.R.get_hl())),
            0xB7 => self.or(self.R.a),
            0xB8 => self.cp(self.R.b),
            0xB9 => self.cp(self.R.c),
            0xBA => self.cp(self.R.d),
            0xBB => self.cp(self.R.e),
            0xBC => self.cp(self.R.h),
            0xBD => self.cp(self.R.l),
            0xBE => self.cp(self.M.read(self.R.get_hl())),
            0xBF => self.cp(self.R.a),
            0xC0 => {
                if self.get(Z) == 0 {
                    self.R.pc = self.pop();
                }
            }
            0xC1 => {
                let v1 = self.pop();
                self.R.set_bc(v1);
            }
            0xC2 => {
                if self.get(Z) == 0 {
                    self.R.pc = d16;
                }
            }
            0xC3 => self.R.pc = d16,
            0xC4 => {
                if self.get(Z) == 0 {
                    self.push(self.R.pc);
                    self.R.pc = d16;
                }
            }
            0xC5 => self.push(self.R.get_bc()),
            0xC6 => self.add(arg1),
            0xC7 => {
                self.push(self.R.pc);
                self.R.pc = 0;
            }
            0xC8 => {
                if self.get(Z) != 0 {
                    self.R.pc = self.pop();
                }
            }
            0xC9 => {
                self.R.pc = self.pop();
            }
            0xCA => {
                if self.get(Z) != 0 {
                    self.R.pc = d16;
                }
            }
            0xCB => {
                let opcode = self.M.read(self.R.pc+1);
                self.R.pc += 2;
                match opcode {
                    0x0 => self.rl(self.R.b, Register::b, true),
                    0x1 => self.rl(self.R.c, Register::c, true),
                    0x2 => self.rl(self.R.d, Register::d, true),
                    0x3 => self.rl(self.R.e, Register::e, true),
                    0x4 => self.rl(self.R.h, Register::h, true),
                    0x5 => self.rl(self.R.l, Register::l, true),
                    0x6 => self.rl(self.M.read(self.R.get_hl()), Register::hl, true),
                    0x7 => self.rr(self.R.a, Register::a, true),
                    0x8 => self.rr(self.R.b, Register::b, true),
                    0x9 => self.rr(self.R.c, Register::c, true),
                    0xA => self.rr(self.R.d, Register::d, true),
                    0xB => self.rr(self.R.e, Register::e, true),
                    0xC => self.rr(self.R.h, Register::h, true),
                    0xD => self.rr(self.R.l, Register::l, true),
                    0xE => self.rr(self.M.read(self.R.get_hl()), Register::hl, true),
                    0xF => self.rr(self.R.a, Register::a, true),
                    0x10 => self.rl(self.R.b, Register::b, false),
                    0x11 => self.rl(self.R.c, Register::c, false),
                    0x12 => self.rl(self.R.d, Register::d, false),
                    0x13 => self.rl(self.R.e, Register::e, false),
                    0x14 => self.rl(self.R.h, Register::h, false),
                    0x15 => self.rl(self.R.l, Register::l, false),
                    0x16 => self.rl(self.M.read(self.R.get_hl()), Register::hl, false),
                    0x17 => self.rl(self.R.a, Register::a, false),
                    0x18 => self.rr(self.R.b, Register::b, false),
                    0x19 => self.rr(self.R.c, Register::c, false),
                    0x1A => self.rr(self.R.d, Register::d, false),
                    0x1B => self.rr(self.R.e, Register::e, false),
                    0x1C => self.rr(self.R.h, Register::h, false),
                    0x1D => self.rr(self.R.l, Register::l, false),
                    0x1E => self.rr(self.M.read(self.R.get_hl()), Register::hl, false),
                    0x1F => self.rr(self.R.a, Register::a, false),
                    0x20 => self.sla(self.R.b, Register::b),
                    0x21 => self.sla(self.R.c, Register::c),
                    0x22 => self.sla(self.R.d, Register::d),
                    0x23 => self.sla(self.R.e, Register::e),
                    0x24 => self.sla(self.R.h, Register::h),
                    0x25 => self.sla(self.R.l, Register::l),
                    0x26 => self.sla(self.M.read(self.R.get_hl()), Register::hl),
                    0x27 => self.sra(self.R.a, Register::a),
                    0x28 => self.sra(self.R.b, Register::b),
                    0x29 => self.sra(self.R.c, Register::c),
                    0x2A => self.sra(self.R.d, Register::d),
                    0x2B => self.sra(self.R.e, Register::e),
                    0x2C => self.sra(self.R.h, Register::h),
                    0x2D => self.sra(self.R.l, Register::l),
                    0x2E => self.sra(self.M.read(self.R.get_hl()), Register::hl),
                    0x2F => self.sra(self.R.a, Register::a),
                    0x30 => self.swap(self.R.b, Register::b),
                    0x31 => self.swap(self.R.c, Register::c),
                    0x32 => self.swap(self.R.d, Register::d),
                    0x33 => self.swap(self.R.e, Register::e),
                    0x34 => self.swap(self.R.h, Register::h),
                    0x35 => self.swap(self.R.l, Register::l),
                    0x36 => self.swap(self.M.read(self.R.get_hl()), Register::hl),
                    0x37 => self.swap(self.R.a, Register::a),
                    0x38 => self.srl(self.R.b, Register::b),
                    0x39 => self.srl(self.R.c, Register::c),
                    0x3A => self.srl(self.R.d, Register::d),
                    0x3B => self.srl(self.R.e, Register::e),
                    0x3C => self.srl(self.R.h, Register::h),
                    0x3D => self.srl(self.R.l, Register::l),
                    0x3E => self.srl(self.M.read(self.R.get_hl()), Register::hl),
                    0x3F => self.srl(self.R.a, Register::a),
                    0x40 => self.bit(0, self.R.b),
                    0x41 => self.bit(0, self.R.c),
                    0x42 => self.bit(0, self.R.d),
                    0x43 => self.bit(0, self.R.e),
                    0x44 => self.bit(0, self.R.h),
                    0x45 => self.bit(0, self.R.l),
                    0x46 => self.bit(0, self.M.read(self.R.get_hl())),
                    0x47 => self.bit(0, self.R.a),
                    0x48 => self.bit(1, self.R.b),
                    0x49 => self.bit(1, self.R.c),
                    0x4A => self.bit(1, self.R.d),
                    0x4B => self.bit(1, self.R.e),
                    0x4C => self.bit(1, self.R.h),
                    0x4D => self.bit(1, self.R.l),
                    0x4E => self.bit(1, self.M.read(self.R.get_hl())),
                    0x4F => self.bit(1, self.R.a),
                    0x50 => self.bit(2, self.R.b),
                    0x51 => self.bit(2, self.R.c),
                    0x52 => self.bit(2, self.R.d),
                    0x53 => self.bit(2, self.R.e),
                    0x54 => self.bit(2, self.R.h),
                    0x55 => self.bit(2, self.R.l),
                    0x56 => self.bit(2, self.M.read(self.R.get_hl())),
                    0x57 => self.bit(2, self.R.a),
                    0x58 => self.bit(3, self.R.b),
                    0x59 => self.bit(3, self.R.c),
                    0x5A => self.bit(3, self.R.d),
                    0x5B => self.bit(3, self.R.e),
                    0x5C => self.bit(3, self.R.h),
                    0x5D => self.bit(3, self.R.l),
                    0x5E => self.bit(3, self.M.read(self.R.get_hl())),
                    0x5F => self.bit(3, self.R.a),
                    0x60 => self.bit(4, self.R.b),
                    0x61 => self.bit(4, self.R.c),
                    0x62 => self.bit(4, self.R.d),
                    0x63 => self.bit(4, self.R.e),
                    0x64 => self.bit(4, self.R.h),
                    0x65 => self.bit(4, self.R.l),
                    0x66 => self.bit(4, self.M.read(self.R.get_hl())),
                    0x67 => self.bit(4, self.R.a),
                    0x68 => self.bit(5, self.R.b),
                    0x69 => self.bit(5, self.R.c),
                    0x6A => self.bit(5, self.R.d),
                    0x6B => self.bit(5, self.R.e),
                    0x6C => self.bit(5, self.R.h),
                    0x6D => self.bit(5, self.R.l),
                    0x6E => self.bit(5, self.M.read(self.R.get_hl())),
                    0x6F => self.bit(5, self.R.a),
                    0x70 => self.bit(6, self.R.b),
                    0x71 => self.bit(6, self.R.c),
                    0x72 => self.bit(6, self.R.d),
                    0x73 => self.bit(6, self.R.e),
                    0x74 => self.bit(6, self.R.h),
                    0x75 => self.bit(6, self.R.l),
                    0x76 => self.bit(6, self.M.read(self.R.get_hl())),
                    0x77 => self.bit(6, self.R.a),
                    0x78 => self.bit(7, self.R.b),
                    0x79 => self.bit(7, self.R.c),
                    0x7A => self.bit(7, self.R.d),
                    0x7B => self.bit(7, self.R.e),
                    0x7C => self.bit(7, self.R.h),
                    0x7D => self.bit(7, self.R.l),
                    0x7E => self.bit(7, self.M.read(self.R.get_hl())),
                    0x7F => self.bit(7, self.R.a),
                    0x80 => self.bitrst(0, self.R.b, Register::b),
                    0x81 => self.bitrst(0, self.R.c, Register::c),
                    0x82 => self.bitrst(0, self.R.d, Register::d),
                    0x83 => self.bitrst(0, self.R.e, Register::e),
                    0x84 => self.bitrst(0, self.R.h, Register::h),
                    0x85 => self.bitrst(0, self.R.l, Register::l),
                    0x86 => self.bitrst(0, self.M.read(self.R.get_hl()), Register::hl),
                    0x87 => self.bitrst(0, self.R.a, Register::a),
                    0x88 => self.bitrst(1, self.R.b, Register::b),
                    0x89 => self.bitrst(1, self.R.c, Register::c),
                    0x8A => self.bitrst(1, self.R.d, Register::d),
                    0x8B => self.bitrst(1, self.R.e, Register::e),
                    0x8C => self.bitrst(1, self.R.h, Register::h),
                    0x8D => self.bitrst(1, self.R.l, Register::l),
                    0x8E => self.bitrst(1, self.M.read(self.R.get_hl()), Register::hl),
                    0x8F => self.bitrst(1, self.R.a, Register::a),
                    0x90 => self.bitrst(2, self.R.b, Register::b),
                    0x91 => self.bitrst(2, self.R.c, Register::c),
                    0x92 => self.bitrst(2, self.R.d, Register::d),
                    0x93 => self.bitrst(2, self.R.e, Register::e),
                    0x94 => self.bitrst(2, self.R.h, Register::h),
                    0x95 => self.bitrst(2, self.R.l, Register::l),
                    0x96 => self.bitrst(2, self.M.read(self.R.get_hl()), Register::hl),
                    0x97 => self.bitrst(2, self.R.a, Register::a),
                    0x98 => self.bitrst(3, self.R.b, Register::b),
                    0x99 => self.bitrst(3, self.R.c, Register::c),
                    0x9A => self.bitrst(3, self.R.d, Register::d),
                    0x9B => self.bitrst(3, self.R.e, Register::e),
                    0x9C => self.bitrst(3, self.R.h, Register::h),
                    0x9D => self.bitrst(3, self.R.l, Register::l),
                    0x9E => self.bitrst(3, self.M.read(self.R.get_hl()), Register::hl),
                    0x9F => self.bitrst(3, self.R.a, Register::a),
                    0xA0 => self.bitrst(4, self.R.b, Register::b),
                    0xA1 => self.bitrst(4, self.R.c, Register::c),
                    0xA2 => self.bitrst(4, self.R.d, Register::d),
                    0xA3 => self.bitrst(4, self.R.e, Register::e),
                    0xA4 => self.bitrst(4, self.R.h, Register::h),
                    0xA5 => self.bitrst(4, self.R.l, Register::l),
                    0xA6 => self.bitrst(4, self.M.read(self.R.get_hl()), Register::hl),
                    0xA7 => self.bitrst(4, self.R.a, Register::a),
                    0xA8 => self.bitrst(5, self.R.b, Register::b),
                    0xA9 => self.bitrst(5, self.R.c, Register::c),
                    0xAA => self.bitrst(5, self.R.d, Register::d),
                    0xAB => self.bitrst(5, self.R.e, Register::e),
                    0xAC => self.bitrst(5, self.R.h, Register::h),
                    0xAD => self.bitrst(5, self.R.l, Register::l),
                    0xAE => self.bitrst(5, self.M.read(self.R.get_hl()), Register::hl),
                    0xAF => self.bitrst(5, self.R.a, Register::a),
                    0xB0 => self.bitrst(6, self.R.b, Register::b),
                    0xB1 => self.bitrst(6, self.R.c, Register::c),
                    0xB2 => self.bitrst(6, self.R.d, Register::d),
                    0xB3 => self.bitrst(6, self.R.e, Register::e),
                    0xB4 => self.bitrst(6, self.R.h, Register::h),
                    0xB5 => self.bitrst(6, self.R.l, Register::l),
                    0xB6 => self.bitrst(6, self.M.read(self.R.get_hl()), Register::hl),
                    0xB7 => self.bitrst(6, self.R.a, Register::a),
                    0xB8 => self.bitrst(7, self.R.b, Register::b),
                    0xB9 => self.bitrst(7, self.R.c, Register::c),
                    0xBA => self.bitrst(7, self.R.d, Register::d),
                    0xBB => self.bitrst(7, self.R.e, Register::e),
                    0xBC => self.bitrst(7, self.R.h, Register::h),
                    0xBD => self.bitrst(7, self.R.l, Register::l),
                    0xBE => self.bitrst(7, self.M.read(self.R.get_hl()), Register::hl),
                    0xBF => self.bitrst(7, self.R.a, Register::a),
                    0xC0 => self.bitset(0, self.R.b, Register::b),
                    0xC1 => self.bitset(0, self.R.c, Register::c),
                    0xC2 => self.bitset(0, self.R.d, Register::d),
                    0xC3 => self.bitset(0, self.R.e, Register::e),
                    0xC4 => self.bitset(0, self.R.h, Register::h),
                    0xC5 => self.bitset(0, self.R.l, Register::l),
                    0xC6 => self.bitset(0, self.M.read(self.R.get_hl()), Register::hl),
                    0xC7 => self.bitset(0, self.R.a, Register::a),
                    0xC8 => self.bitset(1, self.R.b, Register::b),
                    0xC9 => self.bitset(1, self.R.c, Register::c),
                    0xCA => self.bitset(1, self.R.d, Register::d),
                    0xCB => self.bitset(1, self.R.e, Register::e),
                    0xCC => self.bitset(1, self.R.h, Register::h),
                    0xCD => self.bitset(1, self.R.l, Register::l),
                    0xCE => self.bitset(1, self.M.read(self.R.get_hl()), Register::hl),
                    0xCF => self.bitset(1, self.R.a, Register::a),
                    0xD0 => self.bitset(2, self.R.b, Register::b),
                    0xD1 => self.bitset(2, self.R.c, Register::c),
                    0xD2 => self.bitset(2, self.R.d, Register::d),
                    0xD3 => self.bitset(2, self.R.e, Register::e),
                    0xD4 => self.bitset(2, self.R.h, Register::h),
                    0xD5 => self.bitset(2, self.R.l, Register::l),
                    0xD6 => self.bitset(2, self.M.read(self.R.get_hl()), Register::hl),
                    0xD7 => self.bitset(2, self.R.a, Register::a),
                    0xD8 => self.bitset(3, self.R.b, Register::b),
                    0xD9 => self.bitset(3, self.R.c, Register::c),
                    0xDA => self.bitset(3, self.R.d, Register::d),
                    0xDB => self.bitset(3, self.R.e, Register::e),
                    0xDC => self.bitset(3, self.R.h, Register::h),
                    0xDD => self.bitset(3, self.R.l, Register::l),
                    0xDE => self.bitset(3, self.M.read(self.R.get_hl()), Register::hl),
                    0xDF => self.bitset(3, self.R.a, Register::a),
                    0xE0 => self.bitset(4, self.R.b, Register::b),
                    0xE1 => self.bitset(4, self.R.c, Register::c),
                    0xE2 => self.bitset(4, self.R.d, Register::d),
                    0xE3 => self.bitset(4, self.R.e, Register::e),
                    0xE4 => self.bitset(4, self.R.h, Register::h),
                    0xE5 => self.bitset(4, self.R.l, Register::l),
                    0xE6 => self.bitset(4, self.M.read(self.R.get_hl()), Register::hl),
                    0xE7 => self.bitset(4, self.R.a, Register::a),
                    0xE8 => self.bitset(5, self.R.b, Register::b),
                    0xE9 => self.bitset(5, self.R.c, Register::c),
                    0xEA => self.bitset(5, self.R.d, Register::d),
                    0xEB => self.bitset(5, self.R.e, Register::e),
                    0xEC => self.bitset(5, self.R.h, Register::h),
                    0xED => self.bitset(5, self.R.l, Register::l),
                    0xEE => self.bitset(5, self.M.read(self.R.get_hl()), Register::hl),
                    0xEF => self.bitset(5, self.R.a, Register::a),
                    0xF0 => self.bitset(6, self.R.b, Register::b),
                    0xF1 => self.bitset(6, self.R.c, Register::c),
                    0xF2 => self.bitset(6, self.R.d, Register::d),
                    0xF3 => self.bitset(6, self.R.e, Register::e),
                    0xF4 => self.bitset(6, self.R.h, Register::h),
                    0xF5 => self.bitset(6, self.R.l, Register::l),
                    0xF6 => self.bitset(6, self.M.read(self.R.get_hl()), Register::hl),
                    0xF7 => self.bitset(6, self.R.a, Register::a),
                    0xF8 => self.bitset(7, self.R.b, Register::b),
                    0xF9 => self.bitset(7, self.R.c, Register::c),
                    0xFA => self.bitset(7, self.R.d, Register::d),
                    0xFB => self.bitset(7, self.R.e, Register::e),
                    0xFC => self.bitset(7, self.R.h, Register::h),
                    0xFD => self.bitset(7, self.R.l, Register::l),
                    0xFE => self.bitset(7, self.M.read(self.R.get_hl()), Register::hl),
                    0xFF => self.bitset(7, self.R.a, Register::a),
                }
            }
            0xCC => {
                if self.get(Z) != 0 {
                    self.push(self.R.pc);
                    self.R.pc = d16;
                }
            }
            0xCD => {
                self.push(self.R.pc);
                self.R.pc = d16;
            }
            0xCE => {
                self.adc(arg1);
            }
            0xCF => {
                self.push(self.R.pc);
                self.R.pc = 0x08;
            }
            0xD0 => {
                if self.get(C) == 0 {
                    self.R.pc = self.pop();
                }
            }
            0xD1 => {
                let v1 = self.pop();
                self.R.set_de(v1)
            }
            0xD2 => {
                if self.get(C) == 0 {
                    self.R.pc = d16;
                }
            }
            0xD4 => {
                if self.get(C) == 0 {
                    self.push(self.R.pc);
                    self.R.pc = d16;
                }
            }
            0xD5 => self.push(self.R.get_de()),
            0xD6 => self.sub(arg1),
            0xD7 => {
                self.push(self.R.pc);
                self.R.pc = 0x10;
            }
            0xD8 => {
                if self.get(C) != 0 {
                    self.R.pc = self.pop();
                }
            }
            0xD9 => {
                self.R.pc = self.pop();
            }
            0xDA => {
                if self.get(C) != 0 {
                    self.R.pc = d16;
                }
            }
            0xDC => {
                if self.get(C) != 0 {
                    self.push(self.R.pc);
                    self.R.pc = d16;
                }
            }
            0xDE => {
                self.sbc(arg1);
            }
            0xDF => {
                self.push(self.R.pc);
                self.R.pc = 0x18;
            }
            0xE0 => self.M.write((arg1 as u16) | 0xFF00, self.R.a),
            0xE1 => {
                let v1 = self.pop();
                self.R.set_hl(v1)
            }
            0xE2 => self.M.write(self.R.c as u16 | 0xFF00, self.R.a),
            0xE5 => self.push(self.R.get_hl()),
            0xE6 => self.and(arg1),
            0xE7 => {
                self.push(self.R.pc);
                self.R.pc = 0x20;
            }
            0xE8 => {
                self.hcychk16_i(self.R.sp, arg1_i as i16);
                self.cychk16_i(self.R.sp, arg1_i as i16);
                if arg1_i.is_negative() {
                    self.R.pc = self.R.pc.wrapping_sub(arg1_i.abs() as u16);
                } else {
                    self.R.pc = self.R.pc.wrapping_add(arg1_i as u16);
                }
                self.zchk(self.R.sp);
                self.clr(Z);
                self.clr(N);
            }
            0xE9 => self.R.pc = self.R.get_hl(),
            0xEA => self.M.write(d16, self.R.a),
            0xEE => self.xor(arg1),
            0xEF => {
                self.push(self.R.pc);
                self.R.pc = 0x28;
            }
            0xF0 => self.R.a = self.M.read(arg1 as u16 + 0xFF00),
            0xF1 => {
                let v1 = self.pop();
                self.R.set_af(v1)
            }
            0xF2 => self.R.a = self.M.read(self.R.c as u16 | 0xFF00),
            0xF3 => self.S.ime = false,
            0xF5 => self.push(self.R.get_af()),
            0xF6 => self.or(arg1),
            0xF7 => {
                self.push(self.R.pc);
                self.R.pc = 0x30;
            }
            0xF8 => {
                self.cychk16_i(self.R.sp, arg1_i as i16);
                self.hcychk16_i(self.R.sp, arg1_i as i16);
                if arg1_i.is_negative() {
                    self.R.set_hl(self.R.sp - arg1_i.abs() as u16);
                } else {
                    self.R.set_hl(self.R.sp + arg1_i as u16);
                }
                self.clr(Z);
                self.clr(N);
            }
            0xF9 => self.R.sp = self.R.get_hl(),
            0xFA => self.R.a = self.M.read(d16),
            0xFB => self.S.ime = true,
            0xFE => self.cp(arg1),
            0xFF => {
                self.push(self.R.pc);
                self.R.pc = 0x38;
            }
            _ => {}
        }
    }

    fn T(&mut self, cycles: u32) {
        self.S.cycles += cycles;
    }

    pub fn cychk(&mut self, v1: u8, v2: u8) {
        if v1 as u16 + v2 as u16 > 0xFF {
            self.set(C);
        } else {
            self.clr(C);
        }
    }

    pub fn cychk16(&mut self, v1: u16, v2: u16) {
        if v1 as u32 + v2 as u32 > 0xFFFF {
            self.set(C);
        } else {
            self.clr(C);
        }
    }

    pub fn cychk16_i(&mut self, v1: u16, v2: i16) {
        if (v2.is_negative()) {
            self.clr(C);
            return;
        }
        if v1 as u32 + v2 as u32 > 0xFFFF {
            self.set(C);
        } else {
            self.clr(C);
        }
    }

    pub fn hcychk(&mut self, v1: u8, v2: u8) {
        if (v1 & 0xF) + (v2 & 0xF) > 0xF {
            self.set(H);
        } else {
            self.clr(H);
        }
    }

    pub fn hcychk16(&mut self, v1: u16, v2: u16) {
        if (v1 & 0xFF) + (v2 & 0xFF) > 0xFF {
            self.set(H);
        } else {
            self.clr(H);
        }
    }

    pub fn hcychk16_i(&mut self, v1: u16, v2: i16) {
        if v2.is_negative() {
            self.clr(H);
            return;
        }
        if (v1 & 0xFF) as i16 + (v2 & 0xFF as i16) > 0xFF {
            self.set(H);
        } else {
            self.clr(H);
        }
    }

    pub fn brwchk(&mut self, v1: u8, v2: u8) {
        if v1 < v2 {
            self.set(C)
        } else {
            self.clr(C);
        }
    }

    pub fn brwchk16(&mut self, v1: u16, v2: u16) {
        if v1 < v2 {
            self.set(C)
        } else {
            self.clr(C);
        }
    }

    pub fn hbrwchk(&mut self, v1: u8, v2: u8) {
        if (v1 & 0xF) < (v2 & 0xF) {
            self.set(H);
        } else {
            self.clr(H);
        }
    }

    pub fn hbrwchk16(&mut self, v1: u16, v2: u16) {
        if (v1 & 0xFF) < (v2 & 0xFF) {
            self.set(H);
        } else {
            self.clr(H);
        }
    }

    pub fn zchk(&mut self, val: u16) {
        if val == 0 {
            self.set(Z);
        } else {
            self.clr(Z);
        }
    }

    // Core functions

    fn jr(&mut self, cond: bool, val: i8) {
        if (cond) {
            if val.is_negative() {
                self.R.pc -= val.abs() as u16;
            } else {
                self.R.pc += val as u16;
            }
        }
    }

    fn bitset(&mut self, nbit: u8, val: u8, r: Register) {
        let v1 = val | (1 << nbit);
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
    }

    fn bitrst(&mut self, nbit: u8, val: u8, r: Register) {
        let v1 = val & !(1 << nbit);
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
    }

    fn bit(&mut self, nbit: u8, val: u8) {
        if val & (1 << nbit) == 0 {
            self.set(Z);
        } else {
            self.clr(Z);
        }
        self.clr(N);
        self.set(H);
    }

    fn srl(&mut self, val: u8, r: Register) {
        if val & 0x1 > 0 {
            self.set(C);
        } else {
            self.clr(C);
        }
        let v1 = val >> 1;
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
    }
    fn swap(&mut self, val: u8, r: Register) {
        let n1 = (val & 0xF0) >> 4;
        let n2 = (val & 0xF) << 4;
        let v1 = n1 | n2;
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
        self.clr(C);
    }

    fn rl(&mut self, val: u8, r: Register, c: bool) {
        let mut cy: bool = false;
        if self.get(C) != 0 {
            cy = true;
        }

        if val & 0x80 > 0 {
                self.set(C);
        } else {
                self.clr(C);
        }

        let mut v1: u8 = val << 1;
        if c {
            if self.get(C) != 0 {
                v1 |= 1;
            }
        } else {
            if cy {
                v1 |= 1;
            }
        }
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
    }

    fn rr(&mut self, val: u8, r: Register, c: bool) {
        let mut cy: bool = false;
        if c {
            if val & 0x1 > 0 {
                self.set(C);
            } else {
                self.clr(C);
            }
        } else {
            if self.get(C) != 0 {
                cy = true;
            }
        }
        let mut v1: u8 = val >> 1;

        if c {
            if self.get(C) != 0 {
                v1 |= 0x80;
            }
        } else {
            if cy {
                v1 |= 0x80;
            }
        }
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
    }

    fn sla(&mut self, val: u8, r: Register) {
        if val & 0x80 > 0 {
            self.set(C);
        } else {
            self.clr(C);
        }
        let v1 = val << 1;
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
    }

    fn sra(&mut self, val: u8, r: Register) {
        let mut c = false;
        if val & 0x1 > 0 {
            self.set(C);
        } else {
            self.clr(C);
        }
        if val & 0x80 > 0 {
            c = true;
        }
        let mut v1 = val >> 1;
        if c {
            v1 |= 0x80;
        }
        match r {
            Register::b => self.R.b = v1,
            Register::c => self.R.c = v1,
            Register::d => self.R.d = v1,
            Register::e => self.R.e = v1,
            Register::h => self.R.h = v1,
            Register::l => self.R.l = v1,
            Register::hl => self.M.write(self.R.get_hl(), v1),
            Register::a => self.R.b = v1,
            _ => {}
        }
        self.zchk(v1 as u16);
        self.clr(N);
        self.clr(H);
        self.clr(C);
    }

    fn daa(&mut self) {
        if self.get(C) == 0 && self.get(H) == 0 {
            match self.R.a & 0xF {
                0..=9 => match (self.R.a & 0xF0) >> 4 {
                    0..=9 => {
                        self.clr(C);
                    }
                    _ => {
                        self.R.a += 0x60;
                        self.set(C);
                    }
                },
                _ => match (self.R.a & 0xF0) >> 4 {
                    0..=8 => {
                        self.R.a += 0x06;
                        self.clr(C);
                    }
                    _ => {
                        self.R.a += 0x66;
                        self.set(C);
                    }
                },
            }
        } else if self.get(C) == 0 && self.get(H) == 1 {
            match self.R.a & 0xF {
                0..=3 => match (self.R.a & 0xF0) >> 4 {
                    0..=9 => {
                        self.R.a += 0x06;
                        self.clr(C);
                    }
                    _ => {
                        self.R.a += 0x66;
                        self.set(C);
                    }
                },
                6..=0xF => match (self.R.a & 0xF0) >> 4 {
                    0..=8 => {
                        self.R.a += 0xFA;
                        self.clr(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        } else if self.get(C) == 1 && self.get(H) == 0 {
            match self.R.a & 0xF {
                0..=9 => match (self.R.a & 0xF0) >> 4 {
                    0..=2 => {
                        self.R.a += 0x60;
                        self.set(C);
                    }
                    7..=0xF => {
                        self.R.a += 0xA0;
                        self.set(C);
                    }
                    _ => {}
                },
                0xA..=0xF => match (self.R.a & 0xF0) >> 4 {
                    0..=2 => {
                        self.R.a += 0x66;
                        self.set(C);
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            match self.R.a & 0xF {
                0..=3 => match (self.R.a & 0xF0) >> 4 {
                    0..=3 => {
                        self.R.a += 0x66;
                        self.set(C);
                    }
                    _ => {}
                },
                6..=0xF => match (self.R.a & 0xF0) >> 4 {
                    6..=0xF => {
                        self.R.a += 0x9A;
                        self.set(C);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self.zchk(self.R.a as u16);
        self.clr(H);
    }

    fn add16(&mut self, r: Register, ro: Register) {
        let mut v1: u16 = 0;
        match ro {
            Register::bc => v1 = self.R.get_bc(),
            Register::de => v1 = self.R.get_de(),
            Register::hl => v1 = self.R.get_hl(),
            Register::sp => v1 = self.R.get_sp(),
            _ => {}
        }
        let v2: u16;
        match r {
            Register::bc => v2 = self.R.get_bc(),
            Register::de => v2 = self.R.get_de(),
            Register::hl => v2 = self.R.get_hl(),
            Register::sp => v2 = self.R.get_sp(),
            _ => {
                v2 = 0;
            }
        }
        self.hcychk16(v1, v2);
        self.cychk16(v1, v2);
        v1 = v1 + v2;
        match r {
            Register::bc => self.R.set_bc(v1),
            Register::de => self.R.set_de(v1),
            Register::hl => self.R.set_hl(v1),
            Register::sp => self.R.set_sp(v1),
            _ => {}
        }
        self.clr(N);
    }

    fn add(&mut self, val: u8) {
        self.cychk(self.R.a, val);
        self.hcychk(self.R.a, val);
        self.R.a += val;
        self.zchk(self.R.a as u16);
        self.clr(N);
    }

    fn adc(&mut self, val: u8) {
        if self.get(C) > 0 {
            self.cychk(self.R.a, val + 1);
            self.hcychk(self.R.a, val + 1);
            self.R.a += val + 1;
        } else {
            self.cychk(self.R.a, val);
            self.hcychk(self.R.a, val);
            self.R.a += val;
        }
        self.zchk(self.R.a as u16);
        self.clr(N);
    }

    fn sub(&mut self, val: u8) {
        self.brwchk(self.R.a, val);
        self.hbrwchk(self.R.a, val);
        self.R.a -= val;
        self.set(N);
        self.zchk(self.R.a as u16);
    }

    fn sbc(&mut self, val: u8) {
        if self.get(C) > 0 {
            self.brwchk(self.R.a, val - 1);
            self.hbrwchk(self.R.a, val - 1);
            self.R.a -= val - 1;
        } else {
            self.brwchk(self.R.a, val);
            self.hbrwchk(self.R.a, val);
            self.R.a -= val;
        }
        self.zchk(self.R.a as u16);
        self.set(N);
    }

    fn and(&mut self, val: u8) {
        self.R.a &= val;
        self.zchk(self.R.a as u16);
        self.clr(N);
        self.set(H);
        self.clr(C);
    }

    fn xor(&mut self, val: u8) {
        self.R.a ^= val;
        self.zchk(self.R.a as u16);
        self.clr(N);
        self.clr(H);
        self.clr(C);
    }

    fn or(&mut self, val: u8) {
        self.R.a |= val;
        self.zchk(self.R.a as u16);
        self.clr(N);
        self.clr(H);
        self.clr(C);
    }

    fn cp(&mut self, val: u8) {
        self.brwchk(self.R.a, val);
        self.hbrwchk(self.R.a, val);
        if self.R.a == val {
            self.set(Z);
        } else {
            self.clr(Z);
        }
        self.set(N);
    }

    fn inc(&mut self, reg: Register) {
        let mut v1: u8;
        if reg == Register::hl {
            v1 = self.M.read(self.R.get_hl());
        } else {
            v1 = self.GetReg(reg) as u8;
        }
        self.hcychk(v1, 1);
        v1 = v1.wrapping_add(1);
        self.zchk(v1 as u16);
        if reg == Register::hl {
            self.M.write(self.R.get_hl(), v1);
        } else {
            self.WriteReg(reg, v1 as u16);
        }
        self.clr(N);
    }
    fn dec(&mut self, reg: Register) {
        let mut v1: u8;
        if reg == Register::hl {
            v1 = self.M.read(self.R.get_hl());
        } else {
            v1 = self.GetReg(reg) as u8;
        }
        self.hbrwchk(v1, 1);
        v1 = v1.wrapping_sub(1);
        self.zchk(v1 as u16);
        if reg == Register::hl {
            self.M.write(self.R.get_hl(), v1);
        } else {
            self.WriteReg(reg, v1 as u16);
        }
        self.set(C);
    }

    fn pop_(&mut self, reg: Register) {
        let v = self.pop();
        match reg {
            Register::bc => self.R.set_bc(v),
            Register::de => self.R.set_de(v),
            Register::hl => self.R.set_hl(v),
            Register::af => self.R.set_af(v),
            _ => {}
        }
    }

    fn push_(&mut self, val: u16) {
        self.push(val);
    }

    fn call_cc_aa(&mut self, cond: bool, val: u16) {
        if cond {
            self.push(self.R.pc + 3);
            self.R.pc = val;
        }
    }
    fn rst(&mut self, vec: u16) {
        self.push(self.R.pc + 1);
        self.R.pc = vec;
    }

    fn ret(&mut self, cond: bool, int: bool) {
        if cond {
            if int {
                self.S.ime = true;
            }
            self.R.pc = self.pop();
        }
    }

    fn rla(&mut self, c: bool) {
        let cy = self.get(C);
        if self.R.a & 0x80 > 0 {
            self.set(C);
        } else {
            self.clr(C);
        }
        self.R.a <<= 1;

        if c {
            if self.get(C) != 0 {
                self.R.a += 1;
            }
        } else {
            if cy > 0 {
                self.R.a += 1;
            }
        }


        self.clr(Z);
        self.clr(N);
        self.clr(H);
    }

    fn rra(&mut self, c: bool) {
        if c {
            if self.R.a & 0x1 > 0 {
                self.set(C);
            } else {
                self.clr(C);
            }
            self.R.a >>= 1;
            if self.get(C) != 0 {
                self.R.a |= 0x80;
            }
        } else {
            let cy = self.get(C);
            if self.R.a & 0x1 > 0 {
                self.set(C);
            } else {
                self.clr(C);
            }
            self.R.a >>= 1;
            if cy > 0 {
                self.R.a |= 0x80;
            }
        }
        self.clr(Z);
        self.clr(N);
        self.clr(H);
    }

    // Helpers

    fn WriteReg(&mut self, reg: Register, val: u16) {
        match reg {
            Register::a => self.R.a = val as u8,
            Register::b => self.R.b = val as u8,
            Register::c => self.R.c = val as u8,
            Register::d => self.R.d = val as u8,
            Register::e => self.R.e = val as u8,
            Register::h => self.R.h = val as u8,
            Register::l => self.R.l = val as u8,
            Register::af => self.R.set_af(val),
            Register::bc => self.R.set_bc(val),
            Register::de => self.R.set_de(val),
            Register::hl => self.R.set_hl(val),
            Register::sp => self.R.sp = val,
            Register::pc => self.R.pc = val,
            _ => {}
        }
    }

    fn GetReg(&mut self, reg: Register) -> u16 {
        return match reg {
            Register::a => self.R.a as u16,
            Register::b => self.R.b as u16,
            Register::c => self.R.c as u16,
            Register::d => self.R.d as u16,
            Register::e => self.R.e as u16,
            Register::h => self.R.h as u16,
            Register::l => self.R.l as u16,
            Register::bc => self.R.get_bc(),
            Register::de => self.R.get_de(),
            Register::hl => self.R.get_hl(),
            Register::sp => self.R.sp,
            Register::pc => self.R.pc,
            _ => 0,
        };
    }
}

pub const OP_LEN: [u8; 256] = [
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, 1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1,
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1, 2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1,
];
