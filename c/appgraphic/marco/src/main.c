#include "../include/platform.h"
#include "../include/controller.h"
#include "../include/gui.h"

#ifdef OS_WINDOWS
#define WIN32_LEAN_AND_MEAN
#include <windows.h>

// Redirection WinMain → main
int WINAPI WinMain(HINSTANCE hInst, HINSTANCE hPrev, LPSTR cmd, int show)
{
    return main();
}
#endif

int main()
{
    controller_start();
    gui_start("AutoInputTool", 400, 500);
    return 0;
}
