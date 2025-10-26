#![allow(non_camel_case_types)]

unsafe extern "C" {
    pub fn sdk_init_default();
    pub fn sdk_default_led_pin() -> i32;

    pub fn sdk_gpio_init(pin: u32);
    pub fn sdk_gpio_set_dir(pin: u32, out: bool);
    pub fn sdk_gpio_put(pin: u32, value: bool);

    pub fn sdk_delay_us(us: u32);
    pub fn sdk_delay_ms(ms: u32);
}
