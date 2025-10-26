#pragma once
#include <stdbool.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
    void    sdk_init_default(void);
    int32_t sdk_default_led_pin(void);
    void    sdk_gpio_init(uint32_t pin);
    void    sdk_gpio_set_dir(uint32_t pin, bool out);
    void    sdk_gpio_put(uint32_t pin, bool value);

    void     sdk_delay_us(uint32_t us);
    void     sdk_delay_ms(uint32_t ms);
#ifdef __cplusplus
}
#endif