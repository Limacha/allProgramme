#include "../include/input.h"
#include "../include/controller.h"
#include "../include/gui.h"

#include <windows.h>

selected_mode = HOLD;
unsigned int interval = 200;
static char running = 0;
static char nbF6 = 0;

// ========== ACTION PRINCIPALE ==========
static void run_loop()
{
    while (running)
    {
        if (selected_mode == HOLD)
        {
            input_press(selected_input);
        }
        else if (selected_mode = CLICK)
        {
            input_click(selected_input);
            input_sleep(interval);
        }
    }
    input_release(selected_input);
}

// ========== ACCÈS AU STATE ==========
char controller_is_running()
{
    return running;
}

char controller_nbF6()
{
    return nbF6;
}

// ========== TOGGLE COMMUN (gui & raccourcis) ==========
void controller_toggle()
{
    running = !running;

    gui_refresh();
    if (running)
        run_loop(); // Bloquant dans ce thread → GUI utilise un thread séparé
}

DWORD WINAPI controller_thread(LPVOID lp)
{
    controller_toggle(); // exécute ton code
    return 0;
}

void controller_launchThread()
{
    CreateThread(NULL, 0, controller_thread, NULL, 0, NULL);
}

// ========== VERSION WINDOWS (écoute F6) ==========
DWORD WINAPI key_listener(LPVOID lp)
{
    while (1)
    {
        if (GetAsyncKeyState(VK_F6) & 1)
        {
            nbF6++;
            controller_launchThread(); // EXACTEMENT comme cliquer sur le bouton
        }
        input_sleep(50);
    }
    return 0;
}

void controller_start()
{
    CreateThread(NULL, 0, key_listener, NULL, 0, NULL);
}