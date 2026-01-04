#include <windows.h>
#include "../include/input.h"

selected_input = INPUT_MOUSE_LEFT;
selected_vk = 'A';

// Envoie une touche
static void send_key(WORD vk, BOOL down)
{
    INPUT in = {0};
    in.type = INPUT_KEYBOARD;
    in.ki.wVk = vk;
    in.ki.dwFlags = down ? 0 : KEYEVENTF_KEYUP;
    SendInput(1, &in, sizeof(INPUT));
}

// Envoie un clic souris
static void send_mouse(DWORD flag)
{
    INPUT in = {0};
    in.type = INPUT_MOUSE;
    in.mi.dwFlags = flag;
    SendInput(1, &in, sizeof(INPUT));
}

void input_click(InputType t)
{
    if (t == INPUT_KEY)
    {
        send_key(selected_vk, TRUE);
        send_key(selected_vk, FALSE);
    }
    else if (t == INPUT_MOUSE_LEFT)
    {
        send_mouse(MOUSEEVENTF_LEFTDOWN);
        send_mouse(MOUSEEVENTF_LEFTUP);
    }
    else if (t == INPUT_MOUSE_RIGHT)
    {
        send_mouse(MOUSEEVENTF_RIGHTDOWN);
        send_mouse(MOUSEEVENTF_RIGHTUP);
    }
}

void input_press(InputType t)
{
    if (t == INPUT_KEY)
        send_key(selected_vk, TRUE);
    else if (t == INPUT_MOUSE_LEFT)
        send_mouse(MOUSEEVENTF_LEFTDOWN);
    else if (t == INPUT_MOUSE_RIGHT)
        send_mouse(MOUSEEVENTF_RIGHTDOWN);
}

void input_release(InputType t)
{
    if (t == INPUT_KEY)
        send_key(selected_vk, FALSE);
    else if (t == INPUT_MOUSE_LEFT)
        send_mouse(MOUSEEVENTF_LEFTUP);
    else if (t == INPUT_MOUSE_RIGHT)
        send_mouse(MOUSEEVENTF_RIGHTUP);
}

void input_sleep(unsigned int ms)
{
    Sleep(ms);
}
