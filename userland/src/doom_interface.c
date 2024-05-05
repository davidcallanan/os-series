#define DOOM_IMPLEMENTATION 
#include "PureDOOM.h"

int main(int argc, char** argv)
{
    doom_init(argc, argv, 0);
    while (true)
    {
        doom_update();
    }
}