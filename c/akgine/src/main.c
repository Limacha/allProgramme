#include "../include/platform/platform.h"
#include "../include/engine/window.h"
#include "../include/engine/render.h"

// main fonctions for windows
#ifdef OS_WINDOWS
#define WIN32_LEAN_AND_MEAN
#include <windows.h>

// Redirection WinMain → main
int WINAPI WinMain(HINSTANCE hInst, HINSTANCE hPrev, LPSTR cmd, int show)
{
    return main();
}
#endif

int main(void)
{
    initWindow("AKEngine", DEFAULT_WIDTH, DEFAULT_HEIGHT);
    // initRender(0xFFFFFFFF);

    while (isRunning())
    {
        pollEventWindow();
        // updateRender();
        // refreshWindow();
    }

    // closeRender();
    closeWindow();

    return 0;
}