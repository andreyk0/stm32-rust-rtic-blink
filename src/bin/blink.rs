#![deny(unsafe_code)]
#![cfg_attr(not(doc), no_main)]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f1::stm32f103,
            peripherals = true,
            dispatchers = [CAN_RX1, CAN_SCE, EXTI4, FSMC, TAMPER], // Full list in  stm32f1::stm32f103::Interrupt
            )]
mod app {

    use cortex_m::asm;
    //use cortex_m_semihosting::hprintln;

    use stm32_rust_rtic_blink::{consts::*, types::*};
    use stm32f1xx_hal::prelude::*;

    use dwt_systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<SYS_FREQ_HZ>;

    #[shared]
    struct Shared {
        //led: LedPin,
    }

    #[local]
    struct Local {
        led: LedPin,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut core = cx.core;
        let device = cx.device;
        let mut flash = device.FLASH.constrain();
        let rcc = device.RCC.constrain();

        let _clocks = rcc
            .cfgr
            .use_hse(8u32.MHz())
            .sysclk(SYS_FREQ)
            .pclk1(36u32.MHz())
            .freeze(&mut flash.acr);

        //assert!(clocks.usbclk_valid());

        //hprintln!("clocks");

        let mono = DwtSystick::new(&mut core.DCB, core.DWT, core.SYST, SYS_FREQ.to_Hz());

        //hprintln!("timer");

        //let mut gpioa = device.GPIOA.split();
        //let mut gpiob = device.GPIOB.split();
        let mut gpioc = device.GPIOC.split();

        //hprintln!("gpio");

        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        blink::spawn_after(1.secs()).unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle()]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            asm::delay(SYS_FREQ.to_Hz() / 10000);
        }
    }

    #[task(local = [led], priority = 1)]
    fn blink(cx: blink::Context) {
        blink::spawn_at(monotonics::now() + 1.secs()).unwrap();
        cx.local.led.toggle();
    }
}
