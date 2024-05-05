 
#include <stdio.h>
#define DOOM_IMPLEMENT_MALLOC 
#define DOOM_IMPLEMENT_FILE_IO 

/*
void doom_set_malloc(doom_malloc_fn malloc_fn, doom_free_fn free_fn);
void doom_set_file_io(doom_open_fn open_fn,
                      doom_close_fn close_fn,
                      doom_read_fn read_fn,
                      doom_write_fn write_fn,
                      doom_seek_fn seek_fn,
                      doom_tell_fn tell_fn,
                      doom_eof_fn eof_fn);
*/


//#define DOOM_IMPLEMENT_GETENV 
char* mini_getenv(const char* var) {
    return ".";
}

//#define DOOM_IMPLEMENT_PRINT
void mini_print(const char* str) {
    printf(str);
}

//#define DOOM_IMPLEMENT_EXIT 
void mini_exit(int i) {}

//#define DOOM_IMPLEMENT_GETTIME 
void mini_get_time(int* sec, int* usec) {
    *sec = 0;
    *usec = 0;
}


#define DOOM_IMPLEMENTATION 
#include "PureDOOM.h"

int main(int argc, char** argv)
{
    doom_set_getenv(mini_getenv);
    doom_set_print(mini_print);
    doom_set_exit(mini_exit);


    doom_init(argc, argv, 0);
    while (true)
    {
        doom_update();
    }
}

