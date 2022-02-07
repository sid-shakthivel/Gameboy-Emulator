pub enum Operation {
    NOP,
    HALT,
    EI,
    DI

    // ALU
    INC,
    DEC,
    ADD_16,
    RLC,
    RRC,
    RL,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    DAA,

    // JMP
    JR_S8,
    JR_NZ_S8,
    JR_Z_S8,
    JR_NC_S8,
    C_S8,
    JP_Z_A16,
    JP_NC_A16,
    JP_NZ_A16,
    JP_A16,
    JP_C_A16,

    // LD
    LD_

    // CALL
    CALL_NZ_A16,
    CALL_Z_A16,
    CALL_A16,
    CALL_NC_A16,
    CALL_C_A16,

    // RET
    RET_Z,
    RET,
    RET_NZ,
    RET_NC,
    RET_C,
    
    // Other
    RST // For RST get the index and match it up

    // CB
    SET,
    BIT,
    RES,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
}

pub enum Register {
    
}

pub struct Instruction(pub &'static str, pub Operation, pub u8, pub &dyn Fn(u32)); // Name, Operation, Cycles

pub static INSTRUCTIONS: [Instruction; 256] =
[
    Instruction("NOP", Instruction::NOP, 1),
    Instruction("LD_BC_D16")
]



