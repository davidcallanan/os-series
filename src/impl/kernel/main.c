#include "print.h"


void kernel_main()
{
    print_clear();
    print_set_color(PRINT_COLOR_YELLOW, PRINT_COLOR_BLACK);
    print_str("Welcome to My-OS\n");
    print_set_color(PRINT_COLOR_GREEN, PRINT_COLOR_BLACK);
    print_str("\nUser: ");
    test_input();



}