use stm32f1xx_hal::time::*;

pub const SYS_FREQ_HZ: u32 = 72_000_000;
pub const SYS_FREQ: Hertz = Hz(SYS_FREQ_HZ);
