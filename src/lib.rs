use std::error::Error;
use std::thread;
use std::thread::JoinHandle;
use tudelft_nes_ppu::Cpu;

pub use tudelft_nes_ppu::run_cpu_headless_for;

pub trait TestableCpu: Cpu + Sized {
    fn run_ines_rom(rom: &[u8], num_cycles: usize) -> Result<Self, Box<dyn Error + Send>>;
    fn memory_read(&self, address: u16) -> u8;
}

pub fn run_all_tests<T: TestableCpu>() -> Result<(), String> {
    nestest::<T>()?;

    Ok(())
}

fn nestest<T: TestableCpu>() -> Result<(), String> {
    let rom = include_bytes!("roms/nestest.nes");

    let handle = thread::spawn(|| {
        match T::run_ines_rom(rom, 10000) {
            Err(e) => {
                Err(TestError::Custom(e))
            }
            Ok(cpu) => {
                match (cpu.memory_read(0x0002), cpu.memory_read(0x0003)) {
                    (0, _) | (_, 0) => Ok(()),

                    // branch tests
                    // ------------
                    (0x001, _) => Err(TestError::String("BCS failed to branch".into())),
                    (0x002, _) => Err(TestError::String("BCS branched when it shouldn't have".into())),
                    (0x003, _) => Err(TestError::String("BCC branched when it shouldn't have".into())),
                    (0x004, _) => Err(TestError::String("BCC failed to branch".into())),
                    (0x005, _) => Err(TestError::String("BEQ failed to branch".into())),
                    (0x006, _) => Err(TestError::String("BEQ branched when it shouldn't have".into())),
                    (0x007, _) => Err(TestError::String("BNE failed to branch".into())),
                    (0x008, _) => Err(TestError::String("BNE branched when it shouldn't have".into())),
                    (0x009, _) => Err(TestError::String("BVS failed to branch".into())),
                    (0x00A, _) => Err(TestError::String("BVC branched when it shouldn't have".into())),
                    (0x00B, _) => Err(TestError::String("BVC failed to branch".into())),
                    (0x00C, _) => Err(TestError::String("BVS branched when it shouldn't have".into())),
                    (0x00D, _) => Err(TestError::String("BPL failed to branch".into())),
                    (0x00E, _) => Err(TestError::String("BPL branched when it shouldn't have".into())),
                    (0x00F, _) => Err(TestError::String("BMI failed to branch".into())),
                    (0x010, _) => Err(TestError::String("BMI branched when it shouldn't have".into())),

                    // flag tests
                    // ----------
                    (0x011, _) => Err(TestError::String("PHP/flags failure (bits set)".into())),
                    (0x012, _) => Err(TestError::String("PHP/flags failure (bits clear)".into())),
                    (0x013, _) => Err(TestError::String("PHP/flags failure (misc bit states)".into())),
                    (0x014, _) => Err(TestError::String("PLP/flags failure (misc bit states)".into())),
                    (0x015, _) => Err(TestError::String("PLP/flags failure (misc bit states)".into())),
                    (0x016, _) => Err(TestError::String("PHA/PLA failure (PLA didn't affect Z and N properly)".into())),
                    (0x017, _) => Err(TestError::String("PHA/PLA failure (PLA didn't affect Z and N properly)".into())),

                    // immediate instruction tests
                    // ---------------------------
                    (0x018, _) => Err(TestError::String("ORA # failure".into())),
                    (0x019, _) => Err(TestError::String("ORA # failure".into())),
                    (0x01A, _) => Err(TestError::String("AND # failure".into())),
                    (0x01B, _) => Err(TestError::String("AND # failure".into())),
                    (0x01C, _) => Err(TestError::String("EOR # failure".into())),
                    (0x01D, _) => Err(TestError::String("EOR # failure".into())),
                    (0x01E, _) => Err(TestError::String("ADC # failure (overflow/carry problems)".into())),
                    (0x01F, _) => Err(TestError::String("ADC # failure (decimal mode was turned on)".into())),
                    (0x020, _) => Err(TestError::String("ADC # failure".into())),
                    (0x021, _) => Err(TestError::String("ADC # failure".into())),
                    (0x022, _) => Err(TestError::String("ADC # failure".into())),
                    (0x023, _) => Err(TestError::String("LDA # failure (didn't set N and Z correctly)".into())),
                    (0x024, _) => Err(TestError::String("LDA # failure (didn't set N and Z correctly)".into())),
                    (0x025, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x026, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x027, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x028, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x029, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x02A, _) => Err(TestError::String("CMP # failure (messed up flags)".into())),
                    (0x02B, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x02C, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x02D, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x02E, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x02F, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x030, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x031, _) => Err(TestError::String("CPY # failure (messed up flags)".into())),
                    (0x032, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x033, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x034, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x035, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x036, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x037, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x038, _) => Err(TestError::String("CPX # failure (messed up flags)".into())),
                    (0x039, _) => Err(TestError::String("LDX # failure (didn't set N and Z correctly)".into())),
                    (0x03A, _) => Err(TestError::String("LDX # failure (didn't set N and Z correctly)".into())),
                    (0x03B, _) => Err(TestError::String("LDY # failure (didn't set N and Z correctly)".into())),
                    (0x03C, _) => Err(TestError::String("LDY # failure (didn't set N and Z correctly)".into())),
                    (0x03D, _) => Err(TestError::String("compare(s) stored the result in a register (whoops!)".into())),
                    (0x071, _) => Err(TestError::String("SBC # failure".into())),
                    (0x072, _) => Err(TestError::String("SBC # failure".into())),
                    (0x073, _) => Err(TestError::String("SBC # failure".into())),
                    (0x074, _) => Err(TestError::String("SBC # failure".into())),
                    (0x075, _) => Err(TestError::String("SBC # failure".into())),


                    // implied instruction tests
                    // -------------------------
                    (0x03E, _) => Err(TestError::String("INX/DEX/INY/DEY did something bad".into())),
                    (0x03F, _) => Err(TestError::String("INY/DEY messed up overflow or carry".into())),
                    (0x040, _) => Err(TestError::String("INX/DEX messed up overflow or carry".into())),
                    (0x041, _) => Err(TestError::String("TAY did something bad (changed wrong regs, messed up flags)".into())),
                    (0x042, _) => Err(TestError::String("TAX did something bad (changed wrong regs, messed up flags)".into())),
                    (0x043, _) => Err(TestError::String("TYA did something bad (changed wrong regs, messed up flags)".into())),
                    (0x044, _) => Err(TestError::String("TXA did something bad (changed wrong regs, messed up flags)".into())),
                    (0x045, _) => Err(TestError::String("TXS didn't set flags right, or TSX touched flags and it shouldn't have".into())),

                    // stack tests
                    // -----------
                    (0x046, _) => Err(TestError::String("wrong data popped, or data not in right location on stack".into())),
                    (0x047, _) => Err(TestError::String("JSR didn't work as expected".into())),
                    (0x048, _) => Err(TestError::String("RTS/JSR shouldn't have affected flags".into())),
                    (0x049, _) => Err(TestError::String("RTI/RTS didn't work right when return addys/data were manually pushed".into())),

                    // accumulator tests
                    // -----------------
                    (0x04A, _) => Err(TestError::String("LSR A  failed".into())),
                    (0x04B, _) => Err(TestError::String("ASL A  failed".into())),
                    (0x04C, _) => Err(TestError::String("ROR A  failed".into())),
                    (0x04D, _) => Err(TestError::String("ROL A  failed".into())),

                    // (indirect,x) tests
                    // ------------------
                    (0x058, _) => Err(TestError::String("LDA didn't load the data it expected to load".into())),
                    (0x059, _) => Err(TestError::String("STA didn't store the data where it was supposed to".into())),
                    (0x05A, _) => Err(TestError::String("ORA failure".into())),
                    (0x05B, _) => Err(TestError::String("ORA failure".into())),
                    (0x05C, _) => Err(TestError::String("AND failure".into())),
                    (0x05D, _) => Err(TestError::String("AND failure".into())),
                    (0x05E, _) => Err(TestError::String("EOR failure".into())),
                    (0x05F, _) => Err(TestError::String("EOR failure".into())),
                    (0x060, _) => Err(TestError::String("ADC failure".into())),
                    (0x061, _) => Err(TestError::String("ADC failure".into())),
                    (0x062, _) => Err(TestError::String("ADC failure".into())),
                    (0x063, _) => Err(TestError::String("ADC failure".into())),
                    (0x064, _) => Err(TestError::String("ADC failure".into())),
                    (0x065, _) => Err(TestError::String("CMP failure".into())),
                    (0x066, _) => Err(TestError::String("CMP failure".into())),
                    (0x067, _) => Err(TestError::String("CMP failure".into())),
                    (0x068, _) => Err(TestError::String("CMP failure".into())),
                    (0x069, _) => Err(TestError::String("CMP failure".into())),
                    (0x06A, _) => Err(TestError::String("CMP failure".into())),
                    (0x06B, _) => Err(TestError::String("CMP failure".into())),
                    (0x06C, _) => Err(TestError::String("SBC failure".into())),
                    (0x06D, _) => Err(TestError::String("SBC failure".into())),
                    (0x06E, _) => Err(TestError::String("SBC failure".into())),
                    (0x06F, _) => Err(TestError::String("SBC failure".into())),
                    (0x070, _) => Err(TestError::String("SBC failure".into())),

                    // zeropage tests
                    // --------------
                    (0x076, _) => Err(TestError::String("LDA didn't set the flags properly".into())),
                    (0x077, _) => Err(TestError::String("STA affected flags it shouldn't".into())),
                    (0x078, _) => Err(TestError::String("LDY didn't set the flags properly".into())),
                    (0x079, _) => Err(TestError::String("STY affected flags it shouldn't".into())),
                    (0x07A, _) => Err(TestError::String("LDX didn't set the flags properly".into())),
                    (0x07B, _) => Err(TestError::String("STX affected flags it shouldn't".into())),
                    (0x07C, _) => Err(TestError::String("BIT failure".into())),
                    (0x07D, _) => Err(TestError::String("BIT failure".into())),
                    (0x07E, _) => Err(TestError::String("ORA failure".into())),
                    (0x07F, _) => Err(TestError::String("ORA failure".into())),
                    (0x080, _) => Err(TestError::String("AND failure".into())),
                    (0x081, _) => Err(TestError::String("AND failure".into())),
                    (0x082, _) => Err(TestError::String("EOR failure".into())),
                    (0x083, _) => Err(TestError::String("EOR failure".into())),
                    (0x084, _) => Err(TestError::String("ADC failure".into())),
                    (0x085, _) => Err(TestError::String("ADC failure".into())),
                    (0x086, _) => Err(TestError::String("ADC failure".into())),
                    (0x087, _) => Err(TestError::String("ADC failure".into())),
                    (0x088, _) => Err(TestError::String("ADC failure".into())),
                    (0x089, _) => Err(TestError::String("CMP failure".into())),
                    (0x08A, _) => Err(TestError::String("CMP failure".into())),
                    (0x08B, _) => Err(TestError::String("CMP failure".into())),
                    (0x08C, _) => Err(TestError::String("CMP failure".into())),
                    (0x08D, _) => Err(TestError::String("CMP failure".into())),
                    (0x08E, _) => Err(TestError::String("CMP failure".into())),
                    (0x08F, _) => Err(TestError::String("CMP failure".into())),
                    (0x090, _) => Err(TestError::String("SBC failure".into())),
                    (0x091, _) => Err(TestError::String("SBC failure".into())),
                    (0x092, _) => Err(TestError::String("SBC failure".into())),
                    (0x093, _) => Err(TestError::String("SBC failure".into())),
                    (0x094, _) => Err(TestError::String("SBC failure".into())),
                    (0x095, _) => Err(TestError::String("CPX failure".into())),
                    (0x096, _) => Err(TestError::String("CPX failure".into())),
                    (0x097, _) => Err(TestError::String("CPX failure".into())),
                    (0x098, _) => Err(TestError::String("CPX failure".into())),
                    (0x099, _) => Err(TestError::String("CPX failure".into())),
                    (0x09A, _) => Err(TestError::String("CPX failure".into())),
                    (0x09B, _) => Err(TestError::String("CPX failure".into())),
                    (0x09C, _) => Err(TestError::String("CPY failure".into())),
                    (0x09D, _) => Err(TestError::String("CPY failure".into())),
                    (0x09E, _) => Err(TestError::String("CPY failure".into())),
                    (0x09F, _) => Err(TestError::String("CPY failure".into())),
                    (0x0A0, _) => Err(TestError::String("CPY failure".into())),
                    (0x0A1, _) => Err(TestError::String("CPY failure".into())),
                    (0x0A2, _) => Err(TestError::String("CPY failure".into())),
                    (0x0A3, _) => Err(TestError::String("LSR failure".into())),
                    (0x0A4, _) => Err(TestError::String("LSR failure".into())),
                    (0x0A5, _) => Err(TestError::String("ASL failure".into())),
                    (0x0A6, _) => Err(TestError::String("ASL failure".into())),
                    (0x0A7, _) => Err(TestError::String("ROL failure".into())),
                    (0x0A8, _) => Err(TestError::String("ROL failure".into())),
                    (0x0A9, _) => Err(TestError::String("ROR failure".into())),
                    (0x0AA, _) => Err(TestError::String("ROR failure".into())),
                    (0x0AB, _) => Err(TestError::String("INC failure".into())),
                    (0x0AC, _) => Err(TestError::String("INC failure".into())),
                    (0x0AD, _) => Err(TestError::String("DEC failure".into())),
                    (0x0AE, _) => Err(TestError::String("DEC failure".into())),
                    (0x0AF, _) => Err(TestError::String("DEC failure".into())),

                    // Absolute tests
                    // --------------
                    (0x0B0, _) => Err(TestError::String("LDA didn't set the flags properly".into())),
                    (0x0B1, _) => Err(TestError::String("STA affected flags it shouldn't".into())),
                    (0x0B2, _) => Err(TestError::String("LDY didn't set the flags properly".into())),
                    (0x0B3, _) => Err(TestError::String("STY affected flags it shouldn't".into())),
                    (0x0B4, _) => Err(TestError::String("LDX didn't set the flags properly".into())),
                    (0x0B5, _) => Err(TestError::String("STX affected flags it shouldn't".into())),
                    (0x0B6, _) => Err(TestError::String("BIT failure".into())),
                    (0x0B7, _) => Err(TestError::String("BIT failure".into())),
                    (0x0B8, _) => Err(TestError::String("ORA failure".into())),
                    (0x0B9, _) => Err(TestError::String("ORA failure".into())),
                    (0x0BA, _) => Err(TestError::String("AND failure".into())),
                    (0x0BB, _) => Err(TestError::String("AND failure".into())),
                    (0x0BC, _) => Err(TestError::String("EOR failure".into())),
                    (0x0BD, _) => Err(TestError::String("EOR failure".into())),
                    (0x0BE, _) => Err(TestError::String("ADC failure".into())),
                    (0x0BF, _) => Err(TestError::String("ADC failure".into())),
                    (0x0C0, _) => Err(TestError::String("ADC failure".into())),
                    (0x0C1, _) => Err(TestError::String("ADC failure".into())),
                    (0x0C2, _) => Err(TestError::String("ADC failure".into())),
                    (0x0C3, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C4, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C5, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C6, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C7, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C8, _) => Err(TestError::String("CMP failure".into())),
                    (0x0C9, _) => Err(TestError::String("CMP failure".into())),
                    (0x0CA, _) => Err(TestError::String("SBC failure".into())),
                    (0x0CB, _) => Err(TestError::String("SBC failure".into())),
                    (0x0CC, _) => Err(TestError::String("SBC failure".into())),
                    (0x0CD, _) => Err(TestError::String("SBC failure".into())),
                    (0x0CE, _) => Err(TestError::String("SBC failure".into())),
                    (0x0CF, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D0, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D1, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D2, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D3, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D4, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D5, _) => Err(TestError::String("CPX failure".into())),
                    (0x0D6, _) => Err(TestError::String("CPY failure".into())),
                    (0x0D7, _) => Err(TestError::String("CPY failure".into())),
                    (0x0D8, _) => Err(TestError::String("CPY failure".into())),
                    (0x0D9, _) => Err(TestError::String("CPY failure".into())),
                    (0x0DA, _) => Err(TestError::String("CPY failure".into())),
                    (0x0DB, _) => Err(TestError::String("CPY failure".into())),
                    (0x0DC, _) => Err(TestError::String("CPY failure".into())),
                    (0x0DD, _) => Err(TestError::String("LSR failure".into())),
                    (0x0DE, _) => Err(TestError::String("LSR failure".into())),
                    (0x0DF, _) => Err(TestError::String("ASL failure".into())),
                    (0x0E0, _) => Err(TestError::String("ASL failure".into())),
                    (0x0E1, _) => Err(TestError::String("ROR failure".into())),
                    (0x0E2, _) => Err(TestError::String("ROR failure".into())),
                    (0x0E3, _) => Err(TestError::String("ROL failure".into())),
                    (0x0E4, _) => Err(TestError::String("ROL failure".into())),
                    (0x0E5, _) => Err(TestError::String("INC failure".into())),
                    (0x0E6, _) => Err(TestError::String("INC failure".into())),
                    (0x0E7, _) => Err(TestError::String("DEC failure".into())),
                    (0x0E8, _) => Err(TestError::String("DEC failure".into())),
                    (0x0E9, _) => Err(TestError::String("DEC failure".into())),

                    // (indirect),y tests
                    // ------------------
                    (0x0EA, _) => Err(TestError::String("LDA didn't load what it was supposed to".into())),
                    (0x0EB, _) => Err(TestError::String("read location should've wrapped around ffffh to 0000h".into())),
                    (0x0EC, _) => Err(TestError::String("should've wrapped zeropage address".into())),
                    (0x0ED, _) => Err(TestError::String("ORA failure".into())),
                    (0x0EE, _) => Err(TestError::String("ORA failure".into())),
                    (0x0EF, _) => Err(TestError::String("AND failure".into())),
                    (0x0F0, _) => Err(TestError::String("AND failure".into())),
                    (0x0F1, _) => Err(TestError::String("EOR failure".into())),
                    (0x0F2, _) => Err(TestError::String("EOR failure".into())),
                    (0x0F3, _) => Err(TestError::String("ADC failure".into())),
                    (0x0F4, _) => Err(TestError::String("ADC failure".into())),
                    (0x0F5, _) => Err(TestError::String("ADC failure".into())),
                    (0x0F6, _) => Err(TestError::String("ADC failure".into())),
                    (0x0F7, _) => Err(TestError::String("ADC failure".into())),
                    (0x0F8, _) => Err(TestError::String("CMP failure".into())),
                    (0x0F9, _) => Err(TestError::String("CMP failure".into())),
                    (0x0FA, _) => Err(TestError::String("CMP failure".into())),
                    (0x0FB, _) => Err(TestError::String("CMP failure".into())),
                    (0x0FC, _) => Err(TestError::String("CMP failure".into())),
                    (0x0FD, _) => Err(TestError::String("CMP failure".into())),
                    (0x0FE, _) => Err(TestError::String("CMP failure".into())),

                    (_, 0x001) => Err(TestError::String("SBC failure".into())),
                    (_, 0x002) => Err(TestError::String("SBC failure".into())),
                    (_, 0x003) => Err(TestError::String("SBC failure".into())),
                    (_, 0x004) => Err(TestError::String("SBC failure".into())),
                    (_, 0x005) => Err(TestError::String("SBC failure".into())),
                    (_, 0x006) => Err(TestError::String("STA failure".into())),
                    (_, 0x007) => Err(TestError::String("JMP () data reading didn't wrap properly (this fails on a 65C02)".into())),

                    // zeropage,x tests
                    // ----------------
                    (_, 0x008) => Err(TestError::String("LDY,X failure".into())),
                    (_, 0x009) => Err(TestError::String("LDY,X failure".into())),
                    (_, 0x00A) => Err(TestError::String("STY,X failure".into())),
                    (_, 0x00B) => Err(TestError::String("ORA failure".into())),
                    (_, 0x00C) => Err(TestError::String("ORA failure".into())),
                    (_, 0x00D) => Err(TestError::String("AND failure".into())),
                    (_, 0x00E) => Err(TestError::String("AND failure".into())),
                    (_, 0x00F) => Err(TestError::String("EOR failure".into())),
                    (_, 0x010) => Err(TestError::String("EOR failure".into())),
                    (_, 0x011) => Err(TestError::String("ADC failure".into())),
                    (_, 0x012) => Err(TestError::String("ADC failure".into())),
                    (_, 0x013) => Err(TestError::String("ADC failure".into())),
                    (_, 0x014) => Err(TestError::String("ADC failure".into())),
                    (_, 0x015) => Err(TestError::String("ADC failure".into())),
                    (_, 0x016) => Err(TestError::String("CMP failure".into())),
                    (_, 0x017) => Err(TestError::String("CMP failure".into())),
                    (_, 0x018) => Err(TestError::String("CMP failure".into())),
                    (_, 0x019) => Err(TestError::String("CMP failure".into())),
                    (_, 0x01A) => Err(TestError::String("CMP failure".into())),
                    (_, 0x01B) => Err(TestError::String("CMP failure".into())),
                    (_, 0x01C) => Err(TestError::String("CMP failure".into())),
                    (_, 0x01D) => Err(TestError::String("SBC failure".into())),
                    (_, 0x01E) => Err(TestError::String("SBC failure".into())),
                    (_, 0x01F) => Err(TestError::String("SBC failure".into())),
                    (_, 0x020) => Err(TestError::String("SBC failure".into())),
                    (_, 0x021) => Err(TestError::String("SBC failure".into())),
                    (_, 0x022) => Err(TestError::String("LDA failure".into())),
                    (_, 0x023) => Err(TestError::String("LDA failure".into())),
                    (_, 0x024) => Err(TestError::String("STA failure".into())),
                    (_, 0x025) => Err(TestError::String("LSR failure".into())),
                    (_, 0x026) => Err(TestError::String("LSR failure".into())),
                    (_, 0x027) => Err(TestError::String("ASL failure".into())),
                    (_, 0x028) => Err(TestError::String("ASL failure".into())),
                    (_, 0x029) => Err(TestError::String("ROR failure".into())),
                    (_, 0x02A) => Err(TestError::String("ROR failure".into())),
                    (_, 0x02B) => Err(TestError::String("ROL failure".into())),
                    (_, 0x02C) => Err(TestError::String("ROL failure".into())),
                    (_, 0x02D) => Err(TestError::String("INC failure".into())),
                    (_, 0x02E) => Err(TestError::String("INC failure".into())),
                    (_, 0x02F) => Err(TestError::String("DEC failure".into())),
                    (_, 0x030) => Err(TestError::String("DEC failure".into())),
                    (_, 0x031) => Err(TestError::String("DEC failure".into())),
                    (_, 0x032) => Err(TestError::String("LDX,Y failure".into())),
                    (_, 0x033) => Err(TestError::String("LDX,Y failure".into())),
                    (_, 0x034) => Err(TestError::String("STX,Y failure".into())),
                    (_, 0x035) => Err(TestError::String("STX,Y failure".into())),

                    // absolute,y tests
                    // ----------------
                    (_, 0x036) => Err(TestError::String("LDA failure".into())),
                    (_, 0x037) => Err(TestError::String("LDA failure to wrap properly from ffffh to 0000h".into())),
                    (_, 0x038) => Err(TestError::String("LDA failure, page cross".into())),
                    (_, 0x039) => Err(TestError::String("ORA failure".into())),
                    (_, 0x03A) => Err(TestError::String("ORA failure".into())),
                    (_, 0x03B) => Err(TestError::String("AND failure".into())),
                    (_, 0x03C) => Err(TestError::String("AND failure".into())),
                    (_, 0x03D) => Err(TestError::String("EOR failure".into())),
                    (_, 0x03E) => Err(TestError::String("EOR failure".into())),
                    (_, 0x03F) => Err(TestError::String("ADC failure".into())),
                    (_, 0x040) => Err(TestError::String("ADC failure".into())),
                    (_, 0x041) => Err(TestError::String("ADC failure".into())),
                    (_, 0x042) => Err(TestError::String("ADC failure".into())),
                    (_, 0x043) => Err(TestError::String("ADC failure".into())),
                    (_, 0x044) => Err(TestError::String("CMP failure".into())),
                    (_, 0x045) => Err(TestError::String("CMP failure".into())),
                    (_, 0x046) => Err(TestError::String("CMP failure".into())),
                    (_, 0x047) => Err(TestError::String("CMP failure".into())),
                    (_, 0x048) => Err(TestError::String("CMP failure".into())),
                    (_, 0x049) => Err(TestError::String("CMP failure".into())),
                    (_, 0x04A) => Err(TestError::String("CMP failure".into())),
                    (_, 0x04B) => Err(TestError::String("SBC failure".into())),
                    (_, 0x04C) => Err(TestError::String("SBC failure".into())),
                    (_, 0x04D) => Err(TestError::String("SBC failure".into())),
                    (_, 0x04E) => Err(TestError::String("SBC failure".into())),
                    (_, 0x04F) => Err(TestError::String("SBC failure".into())),
                    (_, 0x050) => Err(TestError::String("STA failure".into())),

                    // absolute,x tests
                    // ----------------
                    (_, 0x051) => Err(TestError::String("LDY,X failure".into())),
                    (_, 0x052) => Err(TestError::String("LDY,X failure (didn't page cross)".into())),
                    (_, 0x053) => Err(TestError::String("ORA failure".into())),
                    (_, 0x054) => Err(TestError::String("ORA failure".into())),
                    (_, 0x055) => Err(TestError::String("AND failure".into())),
                    (_, 0x056) => Err(TestError::String("AND failure".into())),
                    (_, 0x057) => Err(TestError::String("EOR failure".into())),
                    (_, 0x058) => Err(TestError::String("EOR failure".into())),
                    (_, 0x059) => Err(TestError::String("ADC failure".into())),
                    (_, 0x05A) => Err(TestError::String("ADC failure".into())),
                    (_, 0x05B) => Err(TestError::String("ADC failure".into())),
                    (_, 0x05C) => Err(TestError::String("ADC failure".into())),
                    (_, 0x05D) => Err(TestError::String("ADC failure".into())),
                    (_, 0x05E) => Err(TestError::String("CMP failure".into())),
                    (_, 0x05F) => Err(TestError::String("CMP failure".into())),
                    (_, 0x060) => Err(TestError::String("CMP failure".into())),
                    (_, 0x061) => Err(TestError::String("CMP failure".into())),
                    (_, 0x062) => Err(TestError::String("CMP failure".into())),
                    (_, 0x063) => Err(TestError::String("CMP failure".into())),
                    (_, 0x064) => Err(TestError::String("CMP failure".into())),
                    (_, 0x065) => Err(TestError::String("SBC failure".into())),
                    (_, 0x066) => Err(TestError::String("SBC failure".into())),
                    (_, 0x067) => Err(TestError::String("SBC failure".into())),
                    (_, 0x068) => Err(TestError::String("SBC failure".into())),
                    (_, 0x069) => Err(TestError::String("SBC failure".into())),
                    (_, 0x06A) => Err(TestError::String("LDA failure".into())),
                    (_, 0x06B) => Err(TestError::String("LDA failure (didn't page cross)".into())),
                    (_, 0x06C) => Err(TestError::String("STA failure".into())),
                    (_, 0x06D) => Err(TestError::String("LSR failure".into())),
                    (_, 0x06E) => Err(TestError::String("LSR failure".into())),
                    (_, 0x06F) => Err(TestError::String("ASL failure".into())),
                    (_, 0x070) => Err(TestError::String("ASL failure".into())),
                    (_, 0x071) => Err(TestError::String("ROR failure".into())),
                    (_, 0x072) => Err(TestError::String("ROR failure".into())),
                    (_, 0x073) => Err(TestError::String("ROL failure".into())),
                    (_, 0x074) => Err(TestError::String("ROL failure".into())),
                    (_, 0x075) => Err(TestError::String("INC failure".into())),
                    (_, 0x076) => Err(TestError::String("INC failure".into())),
                    (_, 0x077) => Err(TestError::String("DEC failure".into())),
                    (_, 0x078) => Err(TestError::String("DEC failure".into())),
                    (_, 0x079) => Err(TestError::String("DEC failure".into())),
                    (_, 0x07A) => Err(TestError::String("LDX,Y failure".into())),
                    (_, 0x07B) => Err(TestError::String("LDX,Y failure".into())),

                    // ------------------------------------
                    //
                    // Invalid opcode tests... all errors are reported in byte 03h unless
                    // specified.
                    //
                    // NOP - "invalid" opcode tests (error byte 02h)
                    // ---------------------------------------------
                    (0x04E, _) => Err(TestError::String("absolute,X NOPs less than 3 bytes long".into())),
                    (0x04F, _) => Err(TestError::String("implied NOPs affects regs/flags".into())),
                    (0x050, _) => Err(TestError::String("ZP,X NOPs less than 2 bytes long".into())),
                    (0x051, _) => Err(TestError::String("absolute NOP less than 3 bytes long".into())),
                    (0x052, _) => Err(TestError::String("ZP NOPs less than 2 bytes long".into())),
                    (0x053, _) => Err(TestError::String("absolute,X NOPs less than 3 bytes long".into())),
                    (0x054, _) => Err(TestError::String("implied NOPs affects regs/flags".into())),
                    (0x055, _) => Err(TestError::String("ZP,X NOPs less than 2 bytes long".into())),
                    (0x056, _) => Err(TestError::String("absolute NOP less than 3 bytes long".into())),
                    (0x057, _) => Err(TestError::String("ZP NOPs less than 2 bytes long".into())),

                    // LAX - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x07C) => Err(TestError::String("LAX (indr,x) failure".into())),
                    (_, 0x07D) => Err(TestError::String("LAX (indr,x) failure".into())),
                    (_, 0x07E) => Err(TestError::String("LAX zeropage failure".into())),
                    (_, 0x07F) => Err(TestError::String("LAX zeropage failure".into())),
                    (_, 0x080) => Err(TestError::String("LAX absolute failure".into())),
                    (_, 0x081) => Err(TestError::String("LAX absolute failure".into())),
                    (_, 0x082) => Err(TestError::String("LAX (indr),y failure".into())),
                    (_, 0x083) => Err(TestError::String("LAX (indr),y failure".into())),
                    (_, 0x084) => Err(TestError::String("LAX zp,y failure".into())),
                    (_, 0x085) => Err(TestError::String("LAX zp,y failure".into())),
                    (_, 0x086) => Err(TestError::String("LAX abs,y failure".into())),
                    (_, 0x087) => Err(TestError::String("LAX abs,y failure".into())),

                    // SAX - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x088) => Err(TestError::String("SAX (indr,x) failure".into())),
                    (_, 0x089) => Err(TestError::String("SAX (indr,x) failure".into())),
                    (_, 0x08A) => Err(TestError::String("SAX zeropage failure".into())),
                    (_, 0x08B) => Err(TestError::String("SAX zeropage failure".into())),
                    (_, 0x08C) => Err(TestError::String("SAX absolute failure".into())),
                    (_, 0x08D) => Err(TestError::String("SAX absolute failure".into())),
                    (_, 0x08E) => Err(TestError::String("SAX zp,y failure".into())),
                    (_, 0x08F) => Err(TestError::String("SAX zp,y failure".into())),

                    // SBC - "invalid" opcode test
                    // ---------------------------
                    (_, 0x090) => Err(TestError::String("SBC failure".into())),
                    (_, 0x091) => Err(TestError::String("SBC failure".into())),
                    (_, 0x092) => Err(TestError::String("SBC failure".into())),
                    (_, 0x093) => Err(TestError::String("SBC failure".into())),
                    (_, 0x094) => Err(TestError::String("SBC failure".into())),

                    // DCP - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x095) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x096) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x097) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x098) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x099) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x09A) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x09B) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x09C) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x09D) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x09E) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x09F) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x0A0) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x0A1) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0A2) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0A3) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0A4) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0A5) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0A6) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0A7) => Err(TestError::String("DCP abs,x failure".into())),
                    (_, 0x0A8) => Err(TestError::String("DCP abs,x failure".into())),
                    (_, 0x0A9) => Err(TestError::String("DCP abs,x failure".into())),

                    // ISB - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x0AA) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x0AB) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x0AC) => Err(TestError::String("DCP (indr,x) failure".into())),
                    (_, 0x0AD) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x0AE) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x0AF) => Err(TestError::String("DCP zeropage failure".into())),
                    (_, 0x0B0) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x0B1) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x0B2) => Err(TestError::String("DCP absolute failure".into())),
                    (_, 0x0B3) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x0B4) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x0B5) => Err(TestError::String("DCP (indr),y failure".into())),
                    (_, 0x0B6) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0B7) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0B8) => Err(TestError::String("DCP zp,x failure".into())),
                    (_, 0x0B9) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0BA) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0BB) => Err(TestError::String("DCP abs,y failure".into())),
                    (_, 0x0BC) => Err(TestError::String("DCP abs,x failure".into())),
                    (_, 0x0BD) => Err(TestError::String("DCP abs,x failure".into())),
                    (_, 0x0BE) => Err(TestError::String("DCP abs,x failure".into())),

                    // SLO - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x0BF) => Err(TestError::String("SLO (indr,x) failure".into())),
                    (_, 0x0C0) => Err(TestError::String("SLO (indr,x) failure".into())),
                    (_, 0x0C1) => Err(TestError::String("SLO (indr,x) failure".into())),
                    (_, 0x0C2) => Err(TestError::String("SLO zeropage failure".into())),
                    (_, 0x0C3) => Err(TestError::String("SLO zeropage failure".into())),
                    (_, 0x0C4) => Err(TestError::String("SLO zeropage failure".into())),
                    (_, 0x0C5) => Err(TestError::String("SLO absolute failure".into())),
                    (_, 0x0C6) => Err(TestError::String("SLO absolute failure".into())),
                    (_, 0x0C7) => Err(TestError::String("SLO absolute failure".into())),
                    (_, 0x0C8) => Err(TestError::String("SLO (indr),y failure".into())),
                    (_, 0x0C9) => Err(TestError::String("SLO (indr),y failure".into())),
                    (_, 0x0CA) => Err(TestError::String("SLO (indr),y failure".into())),
                    (_, 0x0CB) => Err(TestError::String("SLO zp,x failure".into())),
                    (_, 0x0CC) => Err(TestError::String("SLO zp,x failure".into())),
                    (_, 0x0CD) => Err(TestError::String("SLO zp,x failure".into())),
                    (_, 0x0CE) => Err(TestError::String("SLO abs,y failure".into())),
                    (_, 0x0CF) => Err(TestError::String("SLO abs,y failure".into())),
                    (_, 0x0D0) => Err(TestError::String("SLO abs,y failure".into())),
                    (_, 0x0D1) => Err(TestError::String("SLO abs,x failure".into())),
                    (_, 0x0D2) => Err(TestError::String("SLO abs,x failure".into())),
                    (_, 0x0D3) => Err(TestError::String("SLO abs,x failure".into())),

                    // RLA - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x0D4) => Err(TestError::String("RLA (indr,x) failure".into())),
                    (_, 0x0D5) => Err(TestError::String("RLA (indr,x) failure".into())),
                    (_, 0x0D6) => Err(TestError::String("RLA (indr,x) failure".into())),
                    (_, 0x0D7) => Err(TestError::String("RLA zeropage failure".into())),
                    (_, 0x0D8) => Err(TestError::String("RLA zeropage failure".into())),
                    (_, 0x0D9) => Err(TestError::String("RLA zeropage failure".into())),
                    (_, 0x0DA) => Err(TestError::String("RLA absolute failure".into())),
                    (_, 0x0DB) => Err(TestError::String("RLA absolute failure".into())),
                    (_, 0x0DC) => Err(TestError::String("RLA absolute failure".into())),
                    (_, 0x0DD) => Err(TestError::String("RLA (indr),y failure".into())),
                    (_, 0x0DE) => Err(TestError::String("RLA (indr),y failure".into())),
                    (_, 0x0DF) => Err(TestError::String("RLA (indr),y failure".into())),
                    (_, 0x0E0) => Err(TestError::String("RLA zp,x failure".into())),
                    (_, 0x0E1) => Err(TestError::String("RLA zp,x failure".into())),
                    (_, 0x0E2) => Err(TestError::String("RLA zp,x failure".into())),
                    (_, 0x0E3) => Err(TestError::String("RLA abs,y failure".into())),
                    (_, 0x0E4) => Err(TestError::String("RLA abs,y failure".into())),
                    (_, 0x0E5) => Err(TestError::String("RLA abs,y failure".into())),
                    (_, 0x0E6) => Err(TestError::String("RLA abs,x failure".into())),
                    (_, 0x0E7) => Err(TestError::String("RLA abs,x failure".into())),
                    (_, 0x0E8) => Err(TestError::String("RLA abs,x failure".into())),

                    // SRE - "invalid" opcode tests
                    // ----------------------------
                    (_, 0x0E9) => Err(TestError::String("SRE (indr,x) failure".into())),
                    (_, 0x0EA) => Err(TestError::String("SRE (indr,x) failure".into())),
                    (_, 0x0EB) => Err(TestError::String("SRE (indr,x) failure".into())),
                    (_, 0x0EC) => Err(TestError::String("SRE zeropage failure".into())),
                    (_, 0x0ED) => Err(TestError::String("SRE zeropage failure".into())),
                    (_, 0x0EE) => Err(TestError::String("SRE zeropage failure".into())),
                    (_, 0x0EF) => Err(TestError::String("SRE absolute failure".into())),
                    (_, 0x0F0) => Err(TestError::String("SRE absolute failure".into())),
                    (_, 0x0F1) => Err(TestError::String("SRE absolute failure".into())),
                    (_, 0x0F2) => Err(TestError::String("SRE (indr),y failure".into())),
                    (_, 0x0F3) => Err(TestError::String("SRE (indr),y failure".into())),
                    (_, 0x0F4) => Err(TestError::String("SRE (indr),y failure".into())),
                    (_, 0x0F5) => Err(TestError::String("SRE zp,x failure".into())),
                    (_, 0x0F6) => Err(TestError::String("SRE zp,x failure".into())),
                    (_, 0x0F7) => Err(TestError::String("SRE zp,x failure".into())),
                    (_, 0x0F8) => Err(TestError::String("SRE abs,y failure".into())),
                    (_, 0x0F9) => Err(TestError::String("SRE abs,y failure".into())),
                    (_, 0x0FA) => Err(TestError::String("SRE abs,y failure".into())),
                    (_, 0x0FB) => Err(TestError::String("SRE abs,x failure".into())),
                    (_, 0x0FC) => Err(TestError::String("SRE abs,x failure".into())),
                    (_, 0x0FD) => Err(TestError::String("SRE abs,x failure".into())),


                    // RRA - "invalid" opcode tests
                    // ----------------------------
                    // duplicates?
                    // (_, 0x001) => Err(TestError::String("RRA (indr,x) failure".into())),
                    // (_, 0x002) => Err(TestError::String("RRA (indr,x) failure".into())),
                    // (_, 0x003) => Err(TestError::String("RRA (indr,x) failure".into())),
                    // (_, 0x004) => Err(TestError::String("RRA zeropage failure".into())),
                    // (_, 0x005) => Err(TestError::String("RRA zeropage failure".into())),
                    // (_, 0x006) => Err(TestError::String("RRA zeropage failure".into())),
                    // (_, 0x007) => Err(TestError::String("RRA absolute failure".into())),
                    // (_, 0x008) => Err(TestError::String("RRA absolute failure".into())),
                    // (_, 0x009) => Err(TestError::String("RRA absolute failure".into())),
                    // (_, 0x00A) => Err(TestError::String("RRA (indr),y failure".into())),
                    // (_, 0x00B) => Err(TestError::String("RRA (indr),y failure".into())),
                    // (_, 0x00C) => Err(TestError::String("RRA (indr),y failure".into())),
                    // (_, 0x00D) => Err(TestError::String("RRA zp,x failure".into())),
                    // (_, 0x00E) => Err(TestError::String("RRA zp,x failure".into())),
                    // (_, 0x00F) => Err(TestError::String("RRA zp,x failure".into())),
                    // (_, 0x010) => Err(TestError::String("RRA abs,y failure".into())),
                    // (_, 0x011) => Err(TestError::String("RRA abs,y failure".into())),
                    // (_, 0x012) => Err(TestError::String("RRA abs,y failure".into())),
                    // (_, 0x013) => Err(TestError::String("RRA abs,x failure".into())),
                    // (_, 0x014) => Err(TestError::String("RRA abs,x failure".into())),
                    // (_, 0x015) => Err(TestError::String("RRA abs,x failure".into())),

                    (_, _) => Err(TestError::String("unknown failure".into())),
                }
            }
        }
    });

    process_handle("nestest", handle)
}

enum TestError {
    Custom(Box<dyn Error + Send>),
    String(String),
}

fn process_handle(name: &str, handle: JoinHandle<Result<(), TestError>>) -> Result<(), String> {
    match handle.join() { // <- waits for the thread to complete or panic
        Ok(Ok(_)) => {
            Ok(())
        },
        Ok(Err(e)) => match e {
            TestError::Custom(e) => {
                Err(format!("cpu failed while running test {name} with custom error message {e}"))
            }
            TestError::String(e) => {
                Err(format!("cpu didn't pass test {name}: {e}"))
            }
        }
        Err(e) => {
            let err_msg = match (e.downcast_ref::<&str>(), e.downcast_ref::<String>()) {
                (Some(&s), _) => s,
                (_, Some(s)) => s,
                (None, None) => "<No panic info>",
            };

            Err(format!("cpu implementation panicked while running test {name}: {err_msg}"))
        },
    }
}
