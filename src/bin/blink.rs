#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use kear_os::ffi;


#[entry]
fn main() -> ! {
    unsafe { ffi::sdk_init_default(); }

    let led = unsafe { ffi::sdk_default_led_pin() };
    let led = if led >= 0 { led as u32 } else { 25 };

    unsafe {
        ffi::sdk_gpio_init(led);
        ffi::sdk_gpio_set_dir(led, true);

        loop {
            unsafe { ffi::sdk_gpio_put(led, true); }
            ffi::sdk_delay_ms(250);
            unsafe { ffi::sdk_gpio_put(led, false); }
            ffi::sdk_delay_ms(250);
        }
    }
}
