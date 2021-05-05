#include "print.h"
#include "utils.h"
#include "chars.h"
#include <stdbool.h>

const static size_t NUM_COLS = 80;
const static size_t NUM_ROWS = 25;


struct Char 
{
    uint8_t character;
    uint8_t color;
};

bool capslock = false;
bool shift = false;
bool alt = false;
bool ctrl = false;
bool numlock = true;
bool scrolllock = false;

struct Char* buffer = (struct Char*) 0xb8000;
size_t col = 0;
size_t row = 0;
uint8_t color = PRINT_COLOR_WHITE | PRINT_COLOR_BLACK << 4;

void clear_row(size_t row)
{
    struct Char empty = (struct Char) {
        character: ' ',
        color: color,
    };

    for (size_t col = 0; col < NUM_COLS; col++)
    {
        buffer[col + NUM_COLS * row] = empty;
    }
    
}

void print_clear()
{
    for (size_t i = 0; i < NUM_ROWS; i++)
    {
       clear_row(i);
    }
    
}

void print_newline()
{
    col = 0;
    
    if (row < NUM_ROWS)
    {
        row++;
        return;
    }

    for (size_t row = 0; row < NUM_ROWS; row++)
    {
        for (size_t col = 0; col < NUM_COLS; col++)
        {
            struct Char character = buffer[col + NUM_COLS * row];
            buffer[col + NUM_COLS * (row - 1)] = character;
        }
        
    }

    clear_row(NUM_COLS - 1);
    
}

void print_char(char character)
{
    if (character == '\n')
    {
        print_newline();
        return;
    }

    if (col > NUM_COLS)
    {
        print_newline();
    }

    buffer[col + NUM_COLS * row] = (struct Char)
    {
        character: (uint8_t) character,
        color: color,
    };

    col++;
    
}

void print_str(char* str)
{
    for (size_t i = 0; 1; i++)
    {
        char character = (uint8_t) str[i];

        if (character == '\0')
        {
            return;
        }

        print_char(character);

    }
    
}

void print_int(int num)
{
  char str_num[digit_count(num)+1];
  itoa(num, str_num);
  print_str(str_num);
}

uint8 inb(uint16 port)
{
  uint8 ret;
  asm volatile("inb %1, %0" : "=a"(ret) : "d"(port));
  return ret;
}

void outb(uint16 port, uint8 data)
{
  asm volatile("outb %0, %1" : "=a"(data) : "d"(port));
}

char get_input_keycode()
{
  char ch = 0;
  while((ch = inb(KEYBOARD_PORT)) != 0){
    if(ch > 0)
      return ch;
  }
  return ch;
}

/*
keep the cpu busy for doing nothing(nop)
so that io port will not be processed by cpu
here timer can also be used, but lets do this in looping counter
*/
void wait_for_io(uint32 timer_count)
{
  while(1){
    asm volatile("nop");
    timer_count--;
    if(timer_count <= 0)
      break;
    }
}

void sleep(uint32 timer_count)
{
  wait_for_io(timer_count);
}

void test_input()
{
  char ch = 0;
  char keycode = 0;
  do{
    keycode = get_input_keycode();
    if(keycode == KEY_ENTER){
      print_newline();
    }
    else if (keycode == KEY_LEFT_ALT_PRESS)
    {
      //print_str("LEFT ALT");
      alt = true;
    }
    else if (keycode == KEY_LEFT_SHIFT_PRESS)
    {
      //print_str("LEFT SHIFT");
      shift = true;
    }
    else if (keycode == KEY_LEFT_CTRL_PRESS)
    {
      //print_str("LEFT CTRL");
      ctrl = true;
    }
    else if (keycode == KEY_LEFT_ALT_RELEASE)
    {
      //print_str("LEFT ALT UP");
      alt = false;
    }
    else if (keycode == KEY_LEFT_SHIFT_RELEASE)
    {
      //print_str("LEFT SHIFT UP");
      shift = false;
    }
    else if (keycode == KEY_LEFT_CTRL_RELEASE)
    {
      //print_str("LEFT ALT UP");
      ctrl = false;
    }
    else if (keycode == KEY_CAPS_LOCK_PRESS)
    {
      //print_str("CAPS LOCK");
      capslock = !capslock;
      print_str((char) capslock);
    }
    else{
      if (capslock || shift)
      {
        //we're capitalized
        ch = get_ascii_char(keycode);
      }
      else
      {
        ch = get_ascii_char_lower(keycode);
      }

      print_char(ch);
    }
    sleep(0x02FFFFFF);
  }while(ch > 0);
}
/*
char return_input()
{
  static int MAX_INPUT_LEN = 20;
  char ch = 0;
  char chs[MAX_INPUT_LEN];
  char keycode = 0;
  for (size_t i = 0; i < MAX_INPUT_LEN; i++)
  {
    if(keycode == KEY_ENTER){
      print_newline();
      return chs;
    }else{
      ch = get_ascii_char(keycode);
      print_char(ch);
      chs[i] = ch;
    }
    sleep(0x02FFFFFF);
  }
  return chs;
  
}*/

void print_set_color(uint8_t foreground, uint8_t background)
{
    color = foreground + (background << 4);

}