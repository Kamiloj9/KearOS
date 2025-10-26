#include "pico.h"
#include "hardware/clocks.h"
#include "hardware/gpio.h"
#include "hardware/resets.h"
#include "hardware/xosc.h"
#include "hardware/vreg.h"
#include "cshim.h"

static inline void delay_cycles(volatile uint32_t cycles) {
    while (cycles--) {
        __asm volatile("nop");
    }
}

void sdk_init_default(void) {
    vreg_set_voltage(VREG_VOLTAGE_DEFAULT);
    set_sys_clock_khz(150000, true);
}

int32_t sdk_default_led_pin(void) {
#ifdef PICO_DEFAULT_LED_PIN
    return PICO_DEFAULT_LED_PIN;
#else
    return -1;
#endif
}

void sdk_gpio_init(uint32_t pin)               { gpio_init(pin); }
void sdk_gpio_set_dir(uint32_t pin, bool out)  { gpio_set_dir(pin, out); }
void sdk_gpio_put(uint32_t pin, bool value)    { gpio_put(pin, value); }

void sdk_delay_us(uint32_t us) {
    // ~150 MHz -> ~150 cycles / us; use a conservative factor
    // This is approximate; good enough for LED blinking.
    delay_cycles(us * 120u);
}

void sdk_delay_ms(uint32_t ms) {
    while (ms--) sdk_delay_us(1000u);
}
