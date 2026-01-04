#include "gui/gui.h"

#ifdef _WIN32
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
#ifdef _WIN32
    return gui_main_win32();
#else
    return gui_main_wayland();
#endif
}