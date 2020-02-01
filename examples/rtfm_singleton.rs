#![no_main]
#![no_std]

use cortex_m_rtfm;

app! {
    resources: {
        static RX: Rx<USART1>;
        static TX: Tx<USART1>;
    }
}
fn init(p: init::Peripherals) -> init::LateResources {
    // Note that this is now an owned value, not a reference
    let usart1: USART1 = p.device.USART1;
}