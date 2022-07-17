#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    loop {
        // your code goes here
    }
}