//! Processor state stored in the EFLAGS register.

use super::PrivilegeLevel;

/// The EFLAGS register.
bitflags! {
    pub flags EFlags: u32 {
        /// ID Flag (ID)
        const FLAGS_ID = 1 << 21,
        /// Virtual Interrupt Pending (VIP)
        const FLAGS_VIP = 1 << 20,
        /// Virtual Interrupt Flag (VIF)
        const FLAGS_VIF = 1 << 19,
        /// Alignment Check (AC)
        const FLAGS_AC = 1 << 18,
        /// Virtual-8086 Mode (VM)
        const FLAGS_VM = 1 << 17,
        /// Resume Flag (RF)
        const FLAGS_RF = 1 << 16,
        /// Nested Task (NT)
        const FLAGS_NT = 1 << 14,
        /// I/O Privilege Level (IOPL) 0
        const FLAGS_IOPL0 = 0b00 << 12,
        /// I/O Privilege Level (IOPL) 1
        const FLAGS_IOPL1 = 0b01 << 12,
        /// I/O Privilege Level (IOPL) 2
        const FLAGS_IOPL2 = 0b10 << 12,
        /// I/O Privilege Level (IOPL) 3
        const FLAGS_IOPL3 = 0b11 << 12,
        /// Overflow Flag (OF)
        const FLAGS_OF = 1 << 11,
        /// Direction Flag (DF)
        const FLAGS_DF = 1 << 10,
        /// Interrupt Enable Flag (IF)
        const FLAGS_IF = 1 << 9,
        /// Trap Flag (TF)
        const FLAGS_TF = 1 << 8,
        /// Sign Flag (SF)
        const FLAGS_SF = 1 << 7,
        /// Zero Flag (ZF)
        const FLAGS_ZF = 1 << 6,
        /// Auxiliary Carry Flag (AF)
        const FLAGS_AF = 1 << 4,
        /// Parity Flag (PF)
        const FLAGS_PF = 1 << 2,
        /// Bit 1 is always 1.
        const FLAGS_A1 = 1 << 1,
        /// Carry Flag (CF)
        const FLAGS_CF = 1 << 0,
    }
}

impl Flags {
    /// Creates a new Flags entry. Ensures bit 1 is set.
    pub const fn new() -> EFlags {
        FLAGS_A1
    }

    /// Creates a new Flags with the given I/O privilege level.
    pub const fn from_priv(iopl: PrivilegeLevel) -> EFlags {
        EFlags { bits: (iopl as u32) << 12 }
    }
}

pub unsafe fn read() -> EFlags {
    let r: u32;
    asm!("pushfl; popl $0" : "=r"(r) :: "memory");
    EFlags::from_bits_truncate(r)
}

pub unsafe fn set(val: EFlags) {
    asm!("pushl $0; popfl" :: "r"(val.bits()) : "memory" "flags");
}