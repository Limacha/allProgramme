#include "input.h"
#include "files.h"
#include "platform/platform.h"

static int space_pressed = 0;

// // Abstraction : chaque OS implémente ces fonctions
// extern void platformInitInput(void);
// extern void platformProcessInput(void);
// extern int platformIsSpacePressed(void);
// extern int platformIsRunning(void);

void initInput(void)
{
    platformInitInput();
}

void processInput(void)
{
    platformProcessInput();
}

int isSpacePressed(void)
{
    int spacePress = platformIsSpacePressed();
    /*if (spacePress)
    {
        addToFile("jacko.txt", "\nspace: ");
    }*/
    return spacePress;
}

int isRunning(void)
{
    return platformIsRunning();
}
