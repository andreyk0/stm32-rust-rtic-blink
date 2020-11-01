use stm32f1xx_hal::gpio::*;

// BluePill
//pub type LedPin = gpioc::PC13<Output<PushPull>>;

// Maple
pub type LedPin = gpiob::PB1<Output<PushPull>>;
