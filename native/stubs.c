#include <stdarg.h>

// minimal, no-IO stubs to satisfy pico_platform references
int puts(const char *s) {
    (void)s;
    return 0;
}

int vprintf(const char *fmt, va_list ap) {
    (void)fmt; (void)ap;
    return 0;
}

// exit stub: park the core forever
void _exit(int code) {
    (void)code;
    for (;;) {
        __asm volatile ("wfi");
    }
}
