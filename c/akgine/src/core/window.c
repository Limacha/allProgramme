#include "../../include/platform/platform_window.h"
#include "../../include/engine/window.h"

void initWindow(char *windowName, int width, int height)
{
    platformInitWindow(windowName, width, height);
}

void pollEventWindow(void)
{
    platformPollEventWindow();
}

void closeWindow(void) {}

unsigned char isRunning(void)
{
    return platformIsRunning();
}