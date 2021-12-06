use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

/// Represents the information needed to decode an instruction using the opcode.
///
/// Contains:
/// * Assembler Mnemonic (LDA, ADC, etc.)
/// * Addressing mode
/// * Changes to status flags
/// * Cycles
/// * Possible changes to cycles due to page boundaries
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub addr_mode: AddrMode,
    pub cycles: u32,
    pub can_change_cycles: bool,
    pub no_read: bool,

    pub flag_changes: FlagChanges,
}

/// Instruction addressing mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddrMode {
    /// `OPC #$BB`
    Immediate,
    /// `OPC`
    Implied,
    /// `OPC $LLHH`
    Absolute,
    /// `OPC $LLHH,X`
    AbsoluteX,
    /// `OPC $LLHH,Y`
    AbsoluteY,
    /// `OPC A`
    Accumulator,
    /// `OPC ($LLHH)`
    Indirect,
    /// `OPC ($LL,X)`
    IndirectX,
    /// `OPC ($LL),Y`
    IndirectY,
    /// `OPC $BB`
    Relative,
    /// `OPC $LL`
    ZeroPage,
    /// `OPC $LL,X`
    ZeroPageX,
    /// `OPC $LL,Y`
    ZeroPageY,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mnemonic {
    LDA,
    LDX,
    LDY,
    ADC,
    SBC,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    JMP,
    JSR,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    NOP,
    PHA,
    PHP,
    PLA,
    PLP,
    RTI,
    RTS,
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    BIT,
    AND,
    ORA,
    EOR,
    ASL,
    LSR,
    ROL,
    ROR,
    BRK,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mnemonic::LDA => write!(f, "LDA"),
            Mnemonic::LDX => write!(f, "LDX"),
            Mnemonic::LDY => write!(f, "LDY"),
            Mnemonic::ADC => write!(f, "ADC"),
            Mnemonic::SBC => write!(f, "SBC"),
            Mnemonic::STA => write!(f, "STA"),
            Mnemonic::STX => write!(f, "STX"),
            Mnemonic::STY => write!(f, "STY"),
            Mnemonic::TAX => write!(f, "TAX"),
            Mnemonic::TAY => write!(f, "TAY"),
            Mnemonic::TSX => write!(f, "TSX"),
            Mnemonic::TXA => write!(f, "TXA"),
            Mnemonic::TXS => write!(f, "TXS"),
            Mnemonic::TYA => write!(f, "TYA"),
            Mnemonic::JMP => write!(f, "JMP"),
            Mnemonic::JSR => write!(f, "JSR"),
            Mnemonic::BCC => write!(f, "BCC"),
            Mnemonic::BCS => write!(f, "BCS"),
            Mnemonic::BEQ => write!(f, "BEQ"),
            Mnemonic::BMI => write!(f, "BMI"),
            Mnemonic::BNE => write!(f, "BNE"),
            Mnemonic::BPL => write!(f, "BPL"),
            Mnemonic::BVC => write!(f, "BVC"),
            Mnemonic::BVS => write!(f, "BVS"),
            Mnemonic::INC => write!(f, "INC"),
            Mnemonic::INX => write!(f, "INX"),
            Mnemonic::INY => write!(f, "INY"),
            Mnemonic::DEC => write!(f, "DEC"),
            Mnemonic::DEX => write!(f, "DEX"),
            Mnemonic::DEY => write!(f, "DEY"),
            Mnemonic::NOP => write!(f, "NOP"),
            Mnemonic::PHA => write!(f, "PHA"),
            Mnemonic::PHP => write!(f, "PHP"),
            Mnemonic::PLA => write!(f, "PLA"),
            Mnemonic::PLP => write!(f, "PLP"),
            Mnemonic::RTI => write!(f, "RTI"),
            Mnemonic::RTS => write!(f, "RTS"),
            Mnemonic::SEC => write!(f, "SEC"),
            Mnemonic::SED => write!(f, "SED"),
            Mnemonic::SEI => write!(f, "SEI"),
            Mnemonic::CLC => write!(f, "CLC"),
            Mnemonic::CLD => write!(f, "CLD"),
            Mnemonic::CLI => write!(f, "CLI"),
            Mnemonic::CLV => write!(f, "CLV"),
            Mnemonic::CMP => write!(f, "CMP"),
            Mnemonic::CPX => write!(f, "CPX"),
            Mnemonic::CPY => write!(f, "CPY"),
            Mnemonic::BIT => write!(f, "BIT"),
            Mnemonic::AND => write!(f, "AND"),
            Mnemonic::ORA => write!(f, "ORA"),
            Mnemonic::EOR => write!(f, "EOR"),
            Mnemonic::ASL => write!(f, "ASL"),
            Mnemonic::LSR => write!(f, "LSR"),
            Mnemonic::ROL => write!(f, "ROL"),
            Mnemonic::ROR => write!(f, "ROR"),
            Mnemonic::BRK => write!(f, "BRK"),
        }
    }
}

/// Changes to status flags
///
/// Contents are in the order:
///
/// N Z C I D V
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlagChanges(
    pub FlagChange,
    pub FlagChange,
    pub FlagChange,
    pub FlagChange,
    pub FlagChange,
    pub FlagChange,
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlagChange {
    NotModified,
    Modified,
    Set,
    Unset,
}

macro_rules! instruction {
    ($mnem:ident, $addr_mode:ident, $cycles:literal, $flags:ident) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: false,
                no_read: false,
                flag_changes: $flags
            }
        }
    };
    ($mnem:ident, $addr_mode:ident, $cycles:literal *, $flags:ident) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: true,
                no_read: false,
                flag_changes: $flags
            }
        }
    };
    ($mnem:ident, $addr_mode:ident, $cycles:literal, $($flag:tt)+) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: false,
                no_read: false,
                flag_changes: flag_changes!($($flag),+)
            }
        }
    };
    ($mnem:ident, $addr_mode:ident, $cycles:literal *, $($flag:tt)+) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: true,
                no_read: false,
                flag_changes: flag_changes!($($flag),+)
            }
        }
    };
    ($mnem:ident * , $addr_mode:ident, $cycles:literal, $flags:ident) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: false,
                no_read: true,
                flag_changes: $flags
            }
        }
    };
    ($mnem:ident * , $addr_mode:ident, $cycles:literal *, $flags:ident) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: true,
                no_read: true,
                flag_changes: $flags
            }
        }
    };
    ($mnem:ident * , $addr_mode:ident, $cycles:literal, $($flag:tt)+) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: false,
                no_read: true,
                flag_changes: flag_changes!($($flag),+)
            }
        }
    };
    ($mnem:ident * , $addr_mode:ident, $cycles:literal *, $($flag:tt)+) => {
        {
            use Mnemonic::*;
            Instruction {
                mnemonic: $mnem,
                addr_mode: AddrMode::$addr_mode,
                cycles: $cycles,
                can_change_cycles: true,
                no_read: true,
                flag_changes: flag_changes!($($flag),+)
            }
        }
    };
}

macro_rules! flag_changes {
    ($($f:tt),+) => {{
        use FlagChange::*;
        FlagChanges($(flag_change!($f)),+)
    }}
}

macro_rules! flag_change {
    (-) => {
        NotModified
    };
    (+) => {
        Modified
    };
    (1) => {
        Set
    };
    (0) => {
        Unset
    };
}

macro_rules! instruction_set {
    ($($mnem:ident $read_mode:tt, ($($flag:tt)+) : { $($ins:tt),+ }),+) => {{
        use std::collections::HashMap;
        use Mnemonic::*;
        let mut op_to_ins: HashMap<u8, Instruction> = HashMap::new();
        let mut mnem_to_ops: HashMap<Mnemonic, Vec<u8>> = HashMap::new();

        $(
            let mut ops_range = vec![];
            let flags = flag_changes!($($flag),+);
            let no_read = stringify!($read_mode) == "*";
            $(
                let (op, ins) = if no_read {
                    add_instruction!($mnem *, flags, $ins)
                } else {
                    add_instruction!($mnem, flags, $ins)
                };
                ops_range.push(op);
                debug_assert!(!op_to_ins.contains_key(&op));
                op_to_ins.insert(op, ins);
            )+
            mnem_to_ops.insert($mnem, ops_range);
        )+

        (op_to_ins, mnem_to_ops)
    }}
}

macro_rules! add_instruction {
    ($mnem:ident, $flags:ident, {$op:literal, $addr_mode:ident, $cycles:literal}) => {
        ($op, instruction!($mnem, $addr_mode, $cycles, $flags))
    };
    ($mnem:ident, $flags:ident, {$op:literal, $addr_mode:ident, $cycles:literal *}) => {
        ($op, instruction!($mnem, $addr_mode, $cycles *, $flags))
    };
    ($mnem:ident * , $flags:ident, {$op:literal, $addr_mode:ident, $cycles:literal}) => {
        ($op, instruction!($mnem *, $addr_mode, $cycles, $flags))
    };
    ($mnem:ident * , $flags:ident, {$op:literal, $addr_mode:ident, $cycles:literal *}) => {
        ($op, instruction!($mnem *, $addr_mode, $cycles *, $flags))
    }
}

lazy_static! {
    static ref _INS_SET: (HashMap<u8, Instruction>, HashMap<Mnemonic, Vec<u8>>) = instruction_set!{
        // Load instructions
        LDA -, (+ + - - - -) : {
            { 0xA9, Immediate, 2  },
            { 0xA5, ZeroPage,  3  },
            { 0xB5, ZeroPageX, 4  },
            { 0xAD, Absolute,  4  },
            { 0xBD, AbsoluteX, 4* },
            { 0xB9, AbsoluteY, 4* },
            { 0xA1, IndirectX, 6  },
            { 0xB1, IndirectY, 5* }
        },
        LDX -, (+ + - - - -) : {
            { 0xA2, Immediate, 2  },
            { 0xA6, ZeroPage,  3  },
            { 0xB6, ZeroPageY, 4  },
            { 0xAE, Absolute,  4  },
            { 0xBE, AbsoluteY, 4* }
        },
        LDY -, (+ + - - - -) : {
            { 0xA0, Immediate, 2  },
            { 0xA4, ZeroPage,  3  },
            { 0xB4, ZeroPageX, 4  },
            { 0xAC, Absolute,  4  },
            { 0xBC, AbsoluteX, 4* }
        },

        // Arithmetic
        ADC -, (+ + + - - +) : {
            { 0x69, Immediate, 2  },
            { 0x65, ZeroPage,  3  },
            { 0x75, ZeroPageX, 4  },
            { 0x6D, Absolute,  4  },
            { 0x7D, AbsoluteX, 4* },
            { 0x79, AbsoluteY, 4* },
            { 0x61, IndirectX, 6  },
            { 0x71, IndirectY, 5* }
        },
        SBC -, (+ + + - - +) : {
            { 0xE9, Immediate, 2  },
            { 0xE5, ZeroPage,  3  },
            { 0xF5, ZeroPageX, 4  },
            { 0xED, Absolute,  4  },
            { 0xFD, AbsoluteX, 4* },
            { 0xF9, AbsoluteY, 4* },
            { 0xE1, IndirectX, 6  },
            { 0xF1, IndirectY, 5* }
        },

        // Store
        STA *, (- - - - - -) : {
            { 0x85, ZeroPage,  3 },
            { 0x95, ZeroPageX, 4 },
            { 0x8D, Absolute,  4 },
            { 0x9D, AbsoluteX, 5 },
            { 0x99, AbsoluteY, 5 },
            { 0x81, IndirectX, 6 },
            { 0x91, IndirectY, 6 }
        },
        STX *, (- - - - - -) : {
            { 0x86, ZeroPage,  3 },
            { 0x96, ZeroPageY, 4 },
            { 0x8E, Absolute,  4 }
        },
        STY *, (- - - - - -) : {
            { 0x84, ZeroPage,  3 },
            { 0x94, ZeroPageX, 4 },
            { 0x8C, Absolute,  4 }
        },

        // Transfer registers
        TAX -, (+ + - - - -) : {{ 0xAA, Implied, 2 }},
        TAY -, (+ + - - - -) : {{ 0xA8, Implied, 2 }},
        TSX -, (+ + - - - -) : {{ 0xBA, Implied, 2 }},
        TXA -, (+ + - - - -) : {{ 0x8A, Implied, 2 }},
        TXS -, (- - - - - -) : {{ 0x9A, Implied, 2 }},
        TYA -, (+ + - - - -) : {{ 0x98, Implied, 2 }},

        // Jumps
        JMP -, (- - - - - -) : {
            { 0x4C, Absolute, 3 },
            { 0x6C, Indirect, 5 }
        },
        JSR -, (- - - - - -) : {{ 0x20, Absolute, 6 }},

        // Branches
        BCC -, (- - - - - -) : {{ 0x90, Relative, 2* }},
        BCS -, (- - - - - -) : {{ 0xB0, Relative, 2* }},
        BEQ -, (- - - - - -) : {{ 0xF0, Relative, 2* }},
        BMI -, (- - - - - -) : {{ 0x30, Relative, 2* }},
        BNE -, (- - - - - -) : {{ 0xD0, Relative, 2* }},
        BPL -, (- - - - - -) : {{ 0x10, Relative, 2* }},
        BVC -, (- - - - - -) : {{ 0x50, Relative, 2* }},
        BVS -, (- - - - - -) : {{ 0x70, Relative, 2* }},

        // Increment / Decrement
        INC -, (+ + - - - -) : {
            { 0xE6, ZeroPage,  5 },
            { 0xF6, ZeroPageX, 6 },
            { 0xEE, Absolute,  6 },
            { 0xFE, AbsoluteX, 7 }
        },
        INX -, (+ + - - - -) : {{ 0xE8, Implied, 2 }},
        INY -, (+ + - - - -) : {{ 0xC8, Implied, 2 }},
        DEC -, (+ + - - - -) : {
            { 0xC6, ZeroPage,  5 },
            { 0xD6, ZeroPageX, 6 },
            { 0xCE, Absolute,  6 },
            { 0xDE, AbsoluteX, 7 }
        },
        DEX -, (+ + - - - -) : {{ 0xCA, Implied, 2 }},
        DEY -, (+ + - - - -) : {{ 0x88, Implied, 2 }},

        // No-Op
        NOP -, (- - - - - -) : {{ 0xEA, Implied, 2 }},

        // Push / Pull
        PHA -, (- - - - - -) : {{ 0x48, Implied, 3 }},
        PHP -, (- - - - - -) : {{ 0x08, Implied, 3 }},
        PLA -, (+ + - - - -) : {{ 0x68, Implied, 4 }},
        PLP -, (+ + + + + +) : {{ 0x28, Implied, 4 }},

        // Returns
        RTI -, (+ + + + + +) : {{ 0x40, Implied, 6 }},
        RTS -, (- - - - - -) : {{ 0x60, Implied, 6 }},

        // Control Flags
        SEC -, (- - 1 - - -) : {{ 0x38, Implied, 2 }},
        SED -, (- - - - 1 -) : {{ 0xF8, Implied, 2 }},
        SEI -, (- - - 1 - -) : {{ 0x78, Implied, 2 }},
        CLC -, (- - 0 - - -) : {{ 0x18, Implied, 2 }},
        CLD -, (- - - - 0 -) : {{ 0xD8, Implied, 2 }},
        CLI -, (- - - 0 - -) : {{ 0x58, Implied, 2 }},
        CLV -, (- - - - - 0) : {{ 0xB8, Implied, 2 }},

        // Compare
        CMP -, (+ + + - - -) : {
            { 0xC9, Immediate, 2  },
            { 0xC5, ZeroPage,  3  },
            { 0xD5, ZeroPageX, 4  },
            { 0xCD, Absolute,  4  },
            { 0xDD, AbsoluteX, 4* },
            { 0xD9, AbsoluteY, 4* },
            { 0xC1, IndirectX, 6  },
            { 0xD1, IndirectY, 5* }
        },
        CPX -, (+ + + - - -) : {
            { 0xE0, Immediate, 2 },
            { 0xE4, ZeroPage,  3 },
            { 0xEC, Absolute,  4 }
        },
        CPY -, (+ + + - - -) : {
            { 0xC0, Immediate, 2 },
            { 0xC4, ZeroPage,  3 },
            { 0xCC, Absolute,  4 }
        },

        // Bit Test
        BIT -, (+ + - - - +) : {
            { 0x24, ZeroPage, 3 },
            { 0x2C, Absolute, 4 }
        },

        // Bitwise arithmetic
        AND -, (+ + - - - -) : {
            { 0x29, Immediate, 2  },
            { 0x25, ZeroPage,  3  },
            { 0x35, ZeroPageX, 4  },
            { 0x2D, Absolute,  4  },
            { 0x3D, AbsoluteX, 4* },
            { 0x39, AbsoluteY, 4* },
            { 0x21, IndirectX, 6  },
            { 0x31, IndirectY, 5* }
        },
        ORA -, (+ + - - - -) : {
            { 0x09, Immediate, 2  },
            { 0x05, ZeroPage,  3  },
            { 0x15, ZeroPageX, 4  },
            { 0x0D, Absolute,  4  },
            { 0x1D, AbsoluteX, 4* },
            { 0x19, AbsoluteY, 4* },
            { 0x01, IndirectX, 6  },
            { 0x11, IndirectY, 5* }
        },
        EOR -, (+ + - - - -) : {
            { 0x49, Immediate, 2  },
            { 0x45, ZeroPage,  3  },
            { 0x55, ZeroPageX, 4  },
            { 0x4D, Absolute,  4  },
            { 0x5D, AbsoluteX, 4* },
            { 0x59, AbsoluteY, 4* },
            { 0x41, IndirectX, 6  },
            { 0x51, IndirectY, 5* }
        },
        ASL -, (+ + + - - -) : {
            { 0x0A, Accumulator, 2 },
            { 0x06, ZeroPage,    5 },
            { 0x16, ZeroPageX,   6 },
            { 0x0E, Absolute,    6 },
            { 0x1E, AbsoluteX,   7 }
        },
        LSR -, (0 + + - - -) : {
            { 0x4A, Accumulator, 2 },
            { 0x46, ZeroPage,    5 },
            { 0x56, ZeroPageX,   6 },
            { 0x4E, Absolute,    6 },
            { 0x5E, AbsoluteX,   7 }
        },
        ROL -, (+ + + - - -) : {
            { 0x2A, Accumulator, 2 },
            { 0x26, ZeroPage,    5 },
            { 0x36, ZeroPageX,   6 },
            { 0x2E, Absolute,    6 },
            { 0x3E, AbsoluteX,   7 }
        },
        ROR -, (+ + + - - -) : {
            { 0x6A, Accumulator, 2 },
            { 0x66, ZeroPage,    5 },
            { 0x76, ZeroPageX,   6 },
            { 0x6E, Absolute,    6 },
            { 0x7E, AbsoluteX,   7 }
        },

        // Break
        BRK -, (- - - 1 - -) : {{ 0x00, Implied, 7 }}
    };

    #[doc="A map of opcodes to instructions."]
    pub static ref INSTRUCTION_SET: &'static HashMap<u8, Instruction> = &_INS_SET.0;
    #[doc="A map of instruction mnemonics to opcodes."]
    pub static ref MNEMONIC_TO_OPCODES: &'static HashMap<Mnemonic, Vec<u8>> = &_INS_SET.1;
}
