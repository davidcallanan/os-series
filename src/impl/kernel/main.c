#include "print.h"
#include "keyboard.h"
#include "x86_64/rtc.h"

#define KEY_CODE_A 0x1E
#define KEY_CODE_B 0x30
#define KEY_CODE_C 0x2E
#define KEY_CODE_D 0x20
#define KEY_CODE_E 0x12
#define KEY_CODE_F 0x21
#define KEY_CODE_G 0x22
#define KEY_CODE_H 0x23
#define KEY_CODE_I 0x17
#define KEY_CODE_J 0x24
#define KEY_CODE_K 0x25
#define KEY_CODE_L 0x26
#define KEY_CODE_M 0x32
#define KEY_CODE_N 0x31
#define KEY_CODE_O 0x18
#define KEY_CODE_P 0x19
#define KEY_CODE_Q 0x10
#define KEY_CODE_R 0x13
#define KEY_CODE_S 0x1F
#define KEY_CODE_T 0x14
#define KEY_CODE_U 0x16
#define KEY_CODE_V 0x2F
#define KEY_CODE_W 0x11
#define KEY_CODE_X 0x2D
#define KEY_CODE_Y 0x15
#define KEY_CODE_Z 0x2C
#define KEY_CODE_SPACE 0x39
#define KEY_CODE_ENTER 0x1C

char to_ascii(uint16_t code) {
    switch (code) {
        case KEY_CODE_A: return 'A';
        case KEY_CODE_B: return 'B';
        case KEY_CODE_C: return 'C';
        case KEY_CODE_D: return 'D';
        case KEY_CODE_E: return 'E';
        case KEY_CODE_F: return 'F';
        case KEY_CODE_G: return 'G';
        case KEY_CODE_H: return 'H';
        case KEY_CODE_I: return 'I';
        case KEY_CODE_J: return 'J';
        case KEY_CODE_K: return 'K';
        case KEY_CODE_L: return 'L';
        case KEY_CODE_M: return 'M';
        case KEY_CODE_N: return 'N';
        case KEY_CODE_O: return 'O';
        case KEY_CODE_P: return 'P';
        case KEY_CODE_Q: return 'Q';
        case KEY_CODE_R: return 'R';
        case KEY_CODE_S: return 'S';
        case KEY_CODE_T: return 'T';
        case KEY_CODE_U: return 'U';
        case KEY_CODE_V: return 'V';
        case KEY_CODE_W: return 'W';
        case KEY_CODE_X: return 'X';
        case KEY_CODE_Y: return 'Y';
        case KEY_CODE_Z: return 'Z';
        case KEY_CODE_SPACE: return ' ';
        case KEY_CODE_ENTER: return '\n';
    }    
    
    return '?';
}

void handle_input(struct KeyboardEvent event) {
    if (event.type == KEYBOARD_EVENT_TYPE_MAKE) {
        print_set_color(PRINT_COLOR_BLUE, PRINT_COLOR_WHITE);
        print_char(to_ascii(event.code));
    } else if (event.type == KEYBOARD_EVENT_TYPE_BREAK) {
    }
}

void kernel_main() {
    print_clear();
    print_set_color(PRINT_COLOR_YELLOW, PRINT_COLOR_BLACK);
    print_str("Welcome to our 64-bit kernel!");
    
    keyboard_init();
    keyboard_set_handler(handle_input);
    
    uint8_t prev_seconds = 0;
    
    for (uint8_t i = 0; i < 5;) {
        uint8_t seconds = rtc_seconds();
        
        if (seconds != prev_seconds) {
            i++;
            print_set_color(PRINT_COLOR_GREEN, PRINT_COLOR_BLACK);
            print_str("\nSeconds: ");
            print_uint64_dec(seconds);
        }
        
        prev_seconds = seconds;
    }
    
    print_str(" - Seconds loop disabled.\n");
    
    while (1);
}
