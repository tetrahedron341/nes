mod register;
use register::{Register, StatusRegister};
mod instruction;
use instruction::Instruction;
mod memory_interface;
pub use memory_interface::MOS6502Memory;
use bitflags::bitflags;

use super::error::*;

bitflags! {
    pub struct CPUConfig : u16 {
        const DEBUG = 1 << 0;
        const DEBUG_OUTPUT = 1 << 1 | Self::DEBUG.bits;
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct MOS6502 {
    pub A: Register<u8>,
    pub X: Register<u8>,
    pub Y: Register<u8>,
    pub PC: Register<u16>,
    pub S: Register<u8>,
    pub P: StatusRegister,
    reset: bool,
    nmi: bool,
    irq: bool,

    config: CPUConfig
}

impl MOS6502 {
    pub fn new(config: Option<CPUConfig>) -> Self {
        MOS6502 {
            A: Register(0),
            X: Register(0),
            Y: Register(0),
            PC: Register(0),
            S: Register(0),
            P: StatusRegister::empty(),
            reset: false,
            nmi: false,
            irq: false,
            
            config: config.unwrap_or(CPUConfig::empty())
        }
    }
    
    pub fn reset(&mut self) {
        self.reset = true;
    }

    pub fn nmi(&mut self) {
        self.nmi = true;
    }

    pub fn irq(&mut self) {
        self.irq = true;
    }
    
    fn decode_opcode(&self, opcode: u8) -> Result<Instruction> {
        let ins: Option<Instruction> = instruction::INSTRUCTION_SET.get(&opcode).cloned();
        ins.ok_or(Error::invalid_opcode(self.PC.get(), opcode))
    }
    
    /// Executes one instruction, returns the number of cycles executed.
    pub fn tick(&mut self, mmu: &mut dyn MOS6502Memory) -> Result<u32> {
        if self.reset {
            self.PC.set(mmu.read_double(0xFFFC));
            self.P = StatusRegister::I;
            self.reset = false;
        } else if self.nmi {
            self.push_byte(self.PC.hi(), mmu);
            self.push_byte(self.PC.lo(), mmu);
            self.push_byte(self.P.bits() | 0b0010_0000, mmu);
            self.PC.set(mmu.read_double(0xFFFA));
            self.nmi = false;
        } else if self.irq && !self.P.contains(StatusRegister::I) {
            self.push_byte(self.PC.hi(), mmu);
            self.push_byte(self.PC.lo(), mmu);
            self.push_byte(self.P.bits() | 0b0010_0000, mmu);
            self.P.insert(StatusRegister::I);
            self.PC.set(mmu.read_double(0xFFFE));
        }

        let mut pc_temp = self.PC;
        let opcode = fetch_byte(&mut pc_temp, mmu);
        let ins = self.decode_opcode(opcode)?;
        let mut cycles = ins.cycles;
        
        let mut boundary_crossed = false;
        let mut pointer: Option<u16> = None;
        #[allow(unused)]
        let mut raw_arg: Option<u16> = None;
        #[allow(unused_assignments)]
        let argument = {
            use instruction::AddrMode::*;
            match ins.addr_mode {
                Implied => 0, // Doesn't matter, just use 0
                Accumulator => self.A.get(),
                Immediate => {let arg = fetch_byte(&mut pc_temp, mmu); raw_arg = Some(arg as u16); arg},
                Absolute => {
                    let addr = fetch_double(&mut pc_temp, mmu); 
                    raw_arg = Some(addr);
                    pointer = Some(addr); 
                    if !ins.no_read {mmu.read(addr)} else {0}
                },
                AbsoluteX => {
                    let addr = fetch_double(&mut pc_temp, mmu);
                    raw_arg = Some(addr);
                    let sum_addr = addr + self.X.get() as u16;
                    pointer = Some(sum_addr);
                    if sum_addr&0xff00 != addr&0xff00 { boundary_crossed = true; }
                    if !ins.no_read {mmu.read(sum_addr)} else {0}
                },
                AbsoluteY => {
                    let addr = fetch_double(&mut pc_temp, mmu);
                    raw_arg = Some(addr);
                    let sum_addr = addr + self.Y.get() as u16;
                    pointer = Some(sum_addr);
                    if sum_addr&0xff00 != addr&0xff00 { boundary_crossed = true; }
                    if !ins.no_read {mmu.read(sum_addr)} else {0}
                },
                ZeroPage => {
                    let offset = fetch_byte(&mut pc_temp, mmu);
                    raw_arg = Some(offset as u16);
                    pointer = Some(offset as u16);
                    if !ins.no_read {mmu.read(offset as u16)} else {0}
                },
                ZeroPageX => {
                    let offset1 = fetch_byte(&mut pc_temp, mmu);
                    raw_arg = Some(offset1 as u16);
                    let offset2 = offset1.wrapping_add(self.X.get());
                    pointer = Some(offset2 as u16);
                    if !ins.no_read {mmu.read(offset2 as u16)} else {0}
                },
                ZeroPageY => {
                    let offset1 = fetch_byte(&mut pc_temp, mmu);
                    raw_arg = Some(offset1 as u16);
                    let offset2 = offset1.wrapping_add(self.Y.get());
                    pointer = Some(offset2 as u16);
                    if !ins.no_read {mmu.read(offset2 as u16)} else {0}
                },
                Relative => {
                    let offset = fetch_byte(&mut pc_temp, mmu) as i8;
                    raw_arg = Some(offset as u8 as u16);
                    let addr = pc_temp.get().wrapping_add(offset as i16 as u16);
                    pointer = Some(addr as u16);
                    if addr&0xff00 != pc_temp.get()&0xff00 { boundary_crossed = true; }
                    if !ins.no_read {mmu.read(addr as u16)} else {0}
                },
                Indirect => {
                    let addr1 = fetch_double(&mut pc_temp, mmu);
                    raw_arg = Some(addr1);
                    // Indirect addressing does not carry (This causes the JMP Indirect bug)
                    let addr2_lo = mmu.read(addr1);
                    let addr2_hi = mmu.read((addr1 & 0xff00) + ((addr1+1) & 0x00ff));
                    let addr2 = ((addr2_hi as u16) << 8) | (addr2_lo as u16);
                    pointer = Some(addr2);
                    if !ins.no_read {mmu.read(addr2)} else {0}
                },
                IndirectX => {
                    let addr1 = fetch_byte(&mut pc_temp, mmu);
                    raw_arg = Some(addr1 as u16);
                    let addr2 = addr1.wrapping_add(self.X.get());
                    let addr3 = mmu.read_double(addr2 as u16);
                    pointer = Some(addr3);
                    if !ins.no_read {mmu.read(addr3)} else {0}
                },
                IndirectY => {
                    let addr1 = fetch_byte(&mut pc_temp, mmu) as u16;
                    raw_arg = Some(addr1);
                    let addr2 = mmu.read_double(addr1);
                    let addr3 = addr2.wrapping_add(self.Y.get() as u16);
                    pointer = Some(addr3);
                    if addr3&0xff00 != addr2&0xff00 { boundary_crossed = true; }
                    if !ins.no_read {mmu.read(addr3)} else {0}
                }
            }
        };

        if self.config.contains(CPUConfig::DEBUG_OUTPUT) {
            let disassembled_instruction = {
                use instruction::AddrMode::*;
                match ins.addr_mode {
                    Immediate => format!("{}\t#${:02X}", ins.mnemonic, argument),
                    Implied => format!("{}", ins.mnemonic),
                    Accumulator => format!("{}\tA", ins.mnemonic),
                    Absolute => format!("{}\t${:04X}", ins.mnemonic, raw_arg.unwrap()),
                    AbsoluteX => format!("{}\t${:04X},X", ins.mnemonic, raw_arg.unwrap()),
                    AbsoluteY => format!("{}\t${:04X},Y", ins.mnemonic, raw_arg.unwrap()),
                    ZeroPage => format!("{}\t${:02X}", ins.mnemonic, raw_arg.unwrap()),
                    ZeroPageX => format!("{}\t${:02X},X", ins.mnemonic, raw_arg.unwrap()),
                    ZeroPageY => format!("{}\t${:02X},Y", ins.mnemonic, raw_arg.unwrap()),
                    Relative => format!("{}\t${:02X}", ins.mnemonic, raw_arg.unwrap() as i16),
                    Indirect => format!("{}\t(${:04X})", ins.mnemonic, raw_arg.unwrap()),
                    IndirectX => format!("{}\t(${:02X},X)", ins.mnemonic, raw_arg.unwrap()),
                    IndirectY => format!("{}\t(${:02X}),Y", ins.mnemonic, raw_arg.unwrap())
                }
            };
            println!("PC:{:#06X} A:{:#04X} X:{:#04X} Y:{:#04X} S: {:#04X} P:{:08b} \n Opcode: {:#04X}\tDisassembly: {}", 
                self.PC.get(), self.A.get(), self.X.get(), self.Y.get(), self.S.get(), self.P.bits(), 
                opcode, disassembled_instruction);
        }

        self.PC = pc_temp;
        
        use instruction::Mnemonic::*;
        match ins.mnemonic {
            LDA => {
                self.A.set(argument as u8);
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
            },
            LDX => {
                self.X.set(argument as u8);
                self.P.set(StatusRegister::Z, self.X.is_zero());
                self.P.set(StatusRegister::N, self.X.is_neg());
            },
            LDY => {
                self.Y.set(argument as u8);
                self.P.set(StatusRegister::Z, self.Y.is_zero());
                self.P.set(StatusRegister::N, self.Y.is_neg());
            },
            ADC => {
                let old_a = self.A.get();
                let c_in = if self.P.contains(StatusRegister::C) { 1 } else { 0 };
                let new_a = (self.A.get() as u16)
                    .wrapping_add(argument as u16)
                    .wrapping_add(c_in);
                self.A.set(new_a as u8);
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
                self.P.set(StatusRegister::C, new_a > 0xff);
                self.P.set(StatusRegister::V, ((old_a ^ argument) & 0x80) == 0 && ((old_a as u16 ^ new_a) & 0x80) != 0);
            },
            SBC => {
                let old_a = self.A.get();
                let new_a = (self.A.get() as u16)
                    .wrapping_sub(argument as u16)
                    .wrapping_sub(if !self.P.contains(StatusRegister::C) { 1 } else { 0 });
                self.A.set(new_a as u8);
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
                self.P.set(StatusRegister::C, new_a < 0x100);
                self.P.set(StatusRegister::V, ((old_a as u16 ^ new_a) & 0x80) != 0 && ((old_a ^ argument) & 0x80) != 0);
            },
            STA => {
                mmu.write(pointer.unwrap(), self.A.get());
            },
            STX => {
                mmu.write(pointer.unwrap(), self.X.get());
            },
            STY => {
                mmu.write(pointer.unwrap(), self.Y.get());
            },
            TAX => {
                self.X.set(self.A.get());
                self.P.set(StatusRegister::Z, self.X.is_zero());
                self.P.set(StatusRegister::N, self.X.is_neg());
            },
            TAY => {
                self.Y.set(self.A.get());
                self.P.set(StatusRegister::Z, self.Y.is_zero());
                self.P.set(StatusRegister::N, self.Y.is_neg());
            },
            TSX => {
                self.X.set(self.S.get());
                self.P.set(StatusRegister::Z, self.X.is_zero());
                self.P.set(StatusRegister::N, self.X.is_neg());
            },
            TXA => {
                self.A.set(self.X.get());
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
            },
            TXS => {
                self.S.set(self.X.get());
            },
            TYA => {
                self.A.set(self.Y.get());
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
            },
            JMP => {
                self.PC.set(pointer.unwrap());
            },
            JSR => {
                let last_byte = self.PC - 1;
                self.push_byte(last_byte.hi(), mmu);
                self.push_byte(last_byte.lo(), mmu);
                self.PC.set(pointer.unwrap());
            },
            BCS => {if  self.P.contains(StatusRegister::C) { self.PC.set(pointer.unwrap()); } else {}},
            BCC => {if !self.P.contains(StatusRegister::C) { self.PC.set(pointer.unwrap()); } else {}},
            BEQ => {if  self.P.contains(StatusRegister::Z) { self.PC.set(pointer.unwrap()); } else {}},
            BNE => {if !self.P.contains(StatusRegister::Z) { self.PC.set(pointer.unwrap()); } else {}},
            BMI => {if  self.P.contains(StatusRegister::N) { self.PC.set(pointer.unwrap()); } else {}},
            BPL => {if !self.P.contains(StatusRegister::N) { self.PC.set(pointer.unwrap()); } else {}},
            BVS => {if  self.P.contains(StatusRegister::V) { self.PC.set(pointer.unwrap()); } else {}},
            BVC => {if !self.P.contains(StatusRegister::V) { self.PC.set(pointer.unwrap()); } else {}},
            INC => { 
                let pointer = pointer.unwrap();
                let v = mmu.read(pointer).wrapping_add(1);
                mmu.write(pointer, v);
                self.P.set(StatusRegister::N, v&0x80 != 0);
                self.P.set(StatusRegister::Z, v == 0);
            },
            INX => {
                self.X.inc();
                self.P.set(StatusRegister::Z, self.X.is_zero());
                self.P.set(StatusRegister::N, self.X.is_neg());
            },
            INY => {
                self.Y.inc();
                self.P.set(StatusRegister::Z, self.Y.is_zero());
                self.P.set(StatusRegister::N, self.Y.is_neg());
            },
            DEC => {
                let pointer = pointer.unwrap();
                let v = mmu.read(pointer).wrapping_sub(1);
                mmu.write(pointer, v);
                self.P.set(StatusRegister::N, v&0x80 != 0);
                self.P.set(StatusRegister::Z, v == 0);
            },
            DEX => {
                self.X.dec();
                self.P.set(StatusRegister::Z, self.X.is_zero());
                self.P.set(StatusRegister::N, self.X.is_neg());
            },
            DEY => {
                self.Y.dec();
                self.P.set(StatusRegister::Z, self.Y.is_zero());
                self.P.set(StatusRegister::N, self.Y.is_neg());
            },
            NOP => {},
            PHA => {
                self.push_byte(self.A.get(), mmu);
            },
            PHP => {
                self.push_byte((self.P | StatusRegister::B).bits(), mmu);
            },
            PLA => {
                let a = self.pull_byte(mmu);
                self.A.set(a);
                self.P.set(StatusRegister::Z, self.A.is_zero());
                self.P.set(StatusRegister::N, self.A.is_neg());
            },
            PLP => {
                let p = self.pull_byte(mmu) & (0b11001111);
                self.P = StatusRegister::from_bits_truncate(p);
            },
            RTI => {
                let p = self.pull_byte(mmu) & (0b11001111);
                let pcl = self.pull_byte(mmu) as u16;
                let pch = self.pull_byte(mmu) as u16;
                self.P = StatusRegister::from_bits_truncate(p);
                self.PC = Register((pch<<8) | pcl);
            },
            RTS => {
                let lo = self.pull_byte(mmu) as u16;
                let hi = self.pull_byte(mmu) as u16;
                self.PC = Register((hi<<8) | lo) + 1;
            },
            SEC => {
                self.P.insert(StatusRegister::C);
            },
            SED => {
                self.P.insert(StatusRegister::D);
            },
            SEI => {
                self.P.insert(StatusRegister::I);
            },
            CLC => {
                self.P.remove(StatusRegister::C);
            },
            CLD => {
                self.P.remove(StatusRegister::D);
            },
            CLI => {
                self.P.remove(StatusRegister::I);
            },
            CLV => {
                self.P.remove(StatusRegister::V);
            },
            CMP => {
                let r: Register<u8> = self.A - argument;
                self.P.set(StatusRegister::N, r.is_neg());
                self.P.set(StatusRegister::Z, r.is_zero());
                self.P.set(StatusRegister::C, self.A.get() >= argument);
            },
            CPX => {
                let r: Register<u8> = self.X - argument;
                self.P.set(StatusRegister::N, r.is_neg());
                self.P.set(StatusRegister::Z, r.is_zero());
                self.P.set(StatusRegister::C, self.X.get() >= argument);
            },
            CPY => {
                let r: Register<u8> = self.Y - argument;
                self.P.set(StatusRegister::N, r.is_neg());
                self.P.set(StatusRegister::Z, r.is_zero());
                self.P.set(StatusRegister::C, self.Y.get() >= argument);
            },
            AND => {
                self.A.set(self.A.get() & argument);
                self.P.set(StatusRegister::N, self.A.is_neg());
                self.P.set(StatusRegister::Z, self.A.is_zero());
            },
            ORA => {
                self.A.set(self.A.get() | argument);
                self.P.set(StatusRegister::N, self.A.is_neg());
                self.P.set(StatusRegister::Z, self.A.is_zero());
            },
            EOR => {
                self.A.set(self.A.get() ^ argument);
                self.P.set(StatusRegister::N, self.A.is_neg());
                self.P.set(StatusRegister::Z, self.A.is_zero());
            },
            ASL => {
                let c:bool; let n:bool; let z:bool;
                if ins.addr_mode == instruction::AddrMode::Accumulator {
                    c = self.A.get() & 0b10000000 != 0;
                    self.A.set(self.A.get() << 1);
                    n = self.A.is_neg();
                    z = self.A.is_zero();
                } else {
                    c = argument & 0b10000000 != 0;
                    let v = argument << 1;
                    mmu.write(pointer.unwrap(), v);
                    n = v & 0b10000000 != 0;
                    z = v == 0;
                }
                self.P.set(StatusRegister::C, c);
                self.P.set(StatusRegister::Z, z);
                self.P.set(StatusRegister::N, n);
            },
            LSR => {
                let c:bool; let n:bool; let z:bool;
                if ins.addr_mode == instruction::AddrMode::Accumulator {
                    c = self.A.get() & 0b00000001 != 0;
                    self.A.set(self.A.get() >> 1);
                    n = self.A.is_neg();
                    z = self.A.is_zero();
                } else {
                    c = argument & 0b00000001 != 0;
                    let v = argument >> 1;
                    mmu.write(pointer.unwrap(), v);
                    n = v & 0b10000000 != 0;
                    z = v == 0;
                }
                self.P.set(StatusRegister::C, c);
                self.P.set(StatusRegister::Z, z);
                self.P.set(StatusRegister::N, n);
            },
            ROL => {
                let c:bool; let n:bool; let z:bool;
                let c_in = if self.P.contains(StatusRegister::C) {1} else {0};
                if ins.addr_mode == instruction::AddrMode::Accumulator {
                    c = self.A.get() & 0b10000000 != 0;
                    self.A.set((self.A.get() << 1) + c_in);
                    n = self.A.is_neg();
                    z = self.A.is_zero();
                } else {
                    c = argument & 0b10000000 != 0;
                    let v = (argument << 1) + c_in;
                    mmu.write(pointer.unwrap(), v);
                    n = v & 0b10000000 != 0;
                    z = v == 0;
                }
                self.P.set(StatusRegister::C, c);
                self.P.set(StatusRegister::Z, z);
                self.P.set(StatusRegister::N, n);
            },
            ROR => {
                let c:bool; let n:bool; let z:bool;
                let c_in = if self.P.contains(StatusRegister::C) {0b10000000} else {0};
                if ins.addr_mode == instruction::AddrMode::Accumulator {
                    c = self.A.get() & 0b00000001 != 0;
                    self.A.set((self.A.get() >> 1) + c_in);
                    n = self.A.is_neg();
                    z = self.A.is_zero();
                } else {
                    c = argument & 0b00000001 != 0;
                    let v = (argument >> 1) + c_in;
                    mmu.write(pointer.unwrap(), v);
                    n = v & 0b10000000 != 0;
                    z = v == 0;
                }
                self.P.set(StatusRegister::C, c);
                self.P.set(StatusRegister::Z, z);
                self.P.set(StatusRegister::N, n);
            },
            BRK => {
                self.push_byte((self.PC+1).hi(), mmu);
                self.push_byte((self.PC+1).lo(), mmu);
                self.push_byte(self.P.bits() | 0b0011_0000, mmu);
                self.P.insert(StatusRegister::I);
                self.PC.set(mmu.read_double(0xFFFE));
            },
            BIT => {
                let r = self.A.get() & argument;
                self.P.set(StatusRegister::N, argument & 0x80 != 0);
                self.P.set(StatusRegister::V, argument & 0x40 != 0);
                self.P.set(StatusRegister::Z, r == 0);
            },
            //_ => unimplemented!("Opcode {} is not implemented.", ins.mnemonic)
        }
        
        if boundary_crossed && ins.can_change_cycles {
            cycles += 1;
        }
        
        Ok(cycles)
    }
    
    fn push_byte(&mut self, v: u8, mmu: &mut dyn MOS6502Memory) {
        mmu.write(0x0100 + self.S.get() as u16, v);
        self.S.dec();
    }
    fn pull_byte(&mut self, mmu: &mut dyn MOS6502Memory) -> u8 {
        self.S.inc();
        mmu.read(0x0100 + self.S.get() as u16)
    }
}

fn fetch_byte(pc: &mut Register<u16>, mmu: &mut dyn MOS6502Memory) -> u8 {
    let v = mmu.read(pc.get());
    pc.inc();
    v
}

fn fetch_double(pc: &mut Register<u16>, mmu: &mut dyn MOS6502Memory) -> u16 {
    let lo = fetch_byte(pc, mmu);
    let hi = fetch_byte(pc, mmu);
    ((hi as u16) << 8) | (lo as u16)
}
