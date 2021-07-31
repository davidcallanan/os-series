#include "print.h"

typedef __builtin_va_list va_list;
#define va_start(ap, X) __builtin_va_start(ap, X)
#define va_arg(ap, type) __builtin_va_arg(ap, type)
#define va_end(ap) __builtin_va_end(ap)

void _putc(unsigned char c) { print_char((char)c); }

void printf(const char* fmt, ...)
{
    const char digits[] = "0123456789abcdef";
    va_list ap;
    char buf[10];
    char* s;
    unsigned char u;
    int c, i, pad;

    va_start(ap, fmt);
    while ((c = *fmt++) != 0) {
        if (c == '%') {
            c = *fmt++;
            if (c == 'l')
                c = *fmt++;
            switch (c) {
            case 'c':
                _putc(va_arg(ap, int));
                continue;
            case 's':
                s = va_arg(ap, char*);
                if (s == (void*)0)
                    s = (char*)"<NULL>";
                for (; *s; s++)
                    _putc((int)*s);
                continue;
            case '%':
                _putc('%');
                continue;
            case 'd':
            case 'i':
                c = 'u';
            case 'u':
            case 'x':
            case 'X':
                u = va_arg(ap, unsigned);
                s = buf;
                if (c == 'u') {
                    do
                        *s++ = digits[u % 10U];
                    while (u /= 10U);
                } else {
                    pad = 0;
                    for (i = 0; i < 8; i++) {
                        if (pad)
                            *s++ = '0';
                        else {
                            *s++ = digits[u % 16U];
                            if ((u /= 16U) == false)
                                pad = 1;
                        }
                    }
                }
                while (--s >= buf)
                    _putc((int)*s);
                continue;
            }
        }
        _putc((int)c);
    }
    va_end(ap);
}

void kernel_main() {
    print_clear();
    print_set_color(PRINT_COLOR_YELLOW, PRINT_COLOR_BLACK);
    print_str("Welcome to our 64-bit kernel!");
}
