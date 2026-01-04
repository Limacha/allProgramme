#include "../include/gui.h"
#include "../include/controller.h"
#include "../include/input.h"
#include "../resource/resource.h"

#include <windows.h>

// IDs pour les contrôles
#define ID_BTN_START 1
#define ID_RADIO_LEFT 101
#define ID_RADIO_RIGHT 102
#define ID_RADIO_KEY 103
#define ID_BTN_PICKKEY 104
#define ID_RADIO_CLICK 111
#define ID_RADIO_HOLD 112
#define ID_EDIT_INTERVAL 121

// elements de la fenetre
static HWND hwndButton, hwndState;

// choix input
static HWND hRadioLeft, hRadioRight, hRadioKey, hBtnPickKey;

// choix type de click
static HWND hRadioHold, hRadioClick;

// interval
static HWND hEditInterval;

// static HWND hwndClickF6;

// =====================================================================
// REFRESH AFFICHAGE (appelable de n'importe où)
// =====================================================================
void gui_refresh()
{
    if (hwndButton)
        SetWindowText(hwndButton, controller_is_running() ? "Stop" : "Start");

    if (hwndState)
        SetWindowText(hwndState,
                      controller_is_running() ? "Running" : "Stopped");

    if (hRadioClick)
    {
        char txt[32];
        wsprintf(txt, "Touche: %c", (char)selected_vk);
        SetWindowText(hRadioKey, txt);
    }

    if (hEditInterval)
    {
        char txt[10];
        wsprintf(txt, "%u", interval);
        SetWindowText(hEditInterval, txt);
    }

    // if (hwndClickF6)
    // {
    //     char buffer[64];
    //     wsprintf(buffer, "F6 : %u", controller_nbF6());
    //     SetWindowText(hwndClickF6,
    //                   buffer);
    // }
}

static HWND hPickWnd = NULL;
static int picked_key = -1;

LRESULT CALLBACK PickKeyProc(HWND hwnd, UINT msg, WPARAM w, LPARAM l)
{
    switch (msg)
    {
    case WM_KEYDOWN:
    {
        picked_key = (int)w;

        char txt[64];
        wsprintf(txt, "Touche detectee : VK_%d", picked_key);
        SetWindowText(hwnd, txt);

        // ferme la fenêtre après 200ms
        PostMessage(hwnd, WM_CLOSE, 0, 0);
        return 0;
    }

    case WM_CLOSE:
        DestroyWindow(hwnd);
        return 0;

    case WM_DESTROY:
        hPickWnd = NULL;
        return 0;
    }
    return DefWindowProc(hwnd, msg, w, l);
}

int pick_key_dialog(HWND parent)
{
    picked_key = -1;

    // classe
    WNDCLASS wc = {0};
    wc.lpfnWndProc = PickKeyProc;
    wc.lpszClassName = "PickKeyDialog";
    RegisterClass(&wc);

    // créer la mini-fenêtre
    hPickWnd = CreateWindow(
        "PickKeyDialog",
        "Appuyez sur une touche...",
        WS_OVERLAPPED | WS_CAPTION,
        CW_USEDEFAULT, CW_USEDEFAULT,
        260, 80,
        parent, NULL, NULL, NULL);

    ShowWindow(hPickWnd, SW_SHOW);
    UpdateWindow(hPickWnd);

    // boucle modale
    MSG msg;
    while (hPickWnd != NULL)
    {
        while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE))
        {
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
        Sleep(1);
    }

    return picked_key; // contient VK_XXX
}

// ========== CALLBACK WINDOWS ==========
LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM w, LPARAM l)
{
    switch (msg)
    {
    case WM_CREATE:
        hwndButton = CreateWindow("BUTTON", "Start",
                                  WS_VISIBLE | WS_CHILD,
                                  0, 20, 100, 40,
                                  hwnd, (HMENU)ID_BTN_START, NULL, NULL);

        hwndState = CreateWindow("STATIC", "Stopped",
                                 WS_VISIBLE | WS_CHILD,
                                 0, 0, 100, 20,
                                 hwnd, NULL, NULL, NULL);

        // radio buttons group (premier a WS_GROUP)
        hRadioLeft = CreateWindow("BUTTON", "Clic gauche",
                                  WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON | WS_GROUP,
                                  100, 0, 100, 20,
                                  hwnd, (HMENU)ID_RADIO_LEFT, NULL, NULL);

        hRadioRight = CreateWindow("BUTTON", "Clic droit",
                                   WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON,
                                   100, 20, 100, 20,
                                   hwnd, (HMENU)ID_RADIO_RIGHT, NULL, NULL);

        hRadioKey = CreateWindow("BUTTON", "Touche: A",
                                 WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON,
                                 100, 40, 100, 20,
                                 hwnd, (HMENU)ID_RADIO_KEY, NULL, NULL);

        hBtnPickKey = CreateWindow("BUTTON", "Changer...",
                                   WS_VISIBLE | WS_CHILD,
                                   200, 40, 100, 20,
                                   hwnd, (HMENU)ID_BTN_PICKKEY, NULL, NULL);

        // radio group mode
        hRadioHold = CreateWindow("BUTTON", "maintenir",
                                  WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON | WS_GROUP,
                                  200, 0, 100, 20,
                                  hwnd, (HMENU)ID_RADIO_HOLD, NULL, NULL);

        hRadioClick = CreateWindow("BUTTON", "Clicker",
                                   WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON,
                                   200, 20, 100, 20,
                                   hwnd, (HMENU)ID_RADIO_CLICK, NULL, NULL);

        // interval
        hEditInterval = CreateWindow("EDIT", "200",
                                     WS_VISIBLE | WS_CHILD | WS_BORDER | ES_NUMBER,
                                     300, 0, 100, 20,
                                     hwnd, (HMENU)ID_EDIT_INTERVAL, NULL, NULL);

        // set default selection
        CheckRadioButton(hwnd, ID_RADIO_LEFT, ID_RADIO_KEY, ID_RADIO_LEFT);
        selected_input = INPUT_MOUSE_LEFT;

        SendMessage(hRadioHold, BM_SETCHECK, BST_CHECKED, 0);
        selected_mode = HOLD;
        interval = 200;
        // hwndClickF6 = CreateWindow("STATIC", "F6: 0",
        //                            WS_VISIBLE | WS_CHILD,
        //                            0, 50, 150, 20,
        //                            hwnd, NULL, NULL, NULL);
        gui_refresh();
        break;

    case WM_COMMAND:
    {
        int id = LOWORD(w);
        int code = HIWORD(w);

        if (id == ID_EDIT_INTERVAL && code == EN_CHANGE)
        {
            char buf[16];
            GetWindowText(hEditInterval, buf, 16);
            interval = atoi(buf);
        }

        // si il est clicker
        if (code == BN_CLICKED)
        {
            if (id == ID_BTN_START)
            {
                EnableWindow(hwndButton, FALSE);
                controller_launchThread();
                Sleep(200);
                EnableWindow(hwndButton, TRUE);
            }
            // gestion des radios
            else if (id == ID_RADIO_LEFT)
            {
                selected_input = INPUT_MOUSE_LEFT;
                // optionnel : forcer l'état via CheckRadioButton
                CheckRadioButton(hwnd, ID_RADIO_LEFT, ID_RADIO_KEY, ID_RADIO_LEFT);
            }
            else if (id == ID_RADIO_RIGHT)
            {
                selected_input = INPUT_MOUSE_RIGHT;
                CheckRadioButton(hwnd, ID_RADIO_LEFT, ID_RADIO_KEY, ID_RADIO_RIGHT);
            }
            else if (id == ID_RADIO_KEY)
            {
                selected_input = INPUT_KEY;
                CheckRadioButton(hwnd, ID_RADIO_LEFT, ID_RADIO_KEY, ID_RADIO_KEY);
            }
            else if (id == ID_RADIO_HOLD)
            {
                selected_mode = HOLD;
                CheckRadioButton(hwnd, ID_RADIO_HOLD, ID_RADIO_CLICK, ID_RADIO_HOLD);
            }
            else if (id == ID_RADIO_CLICK)
            {
                selected_mode = CLICK;
                CheckRadioButton(hwnd, ID_RADIO_HOLD, ID_RADIO_CLICK, ID_RADIO_CLICK);
            }
            else if (id == ID_BTN_PICKKEY)
            {
                int vk = pick_key_dialog(hwnd);
                selected_vk = vk;

                char txt[32];
                wsprintf(txt, "Touche: %c", (char)vk);
                SetWindowText(hRadioKey, txt);
            }
            // mettre à jour l'affichage si besoin
            gui_refresh();
        }
    }
    break;

        // case WM_TIMER:
        //     gui_refresh();
        //     break;

    case WM_DESTROY:
        PostQuitMessage(0);
        break;

    default:
        return DefWindowProc(hwnd, msg, w, l);
    }
    return 0;
}

// ========== LANCEMENT DE LA GUI ==========
void gui_start(char *windowName, int width, int height)
{
    WNDCLASS wc = {0};
    wc.lpfnWndProc = WndProc;
    wc.lpszClassName = "AutoInput";
    wc.hIcon = LoadIcon(GetModuleHandle(NULL), MAKEINTRESOURCE(IDI_APP_ICON));
    // wc.hIconSm = wc.hIcon;
    RegisterClass(&wc);

    DWORD style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

    HWND hwnd = CreateWindow("AutoInput", windowName,
                             style,
                             CW_USEDEFAULT, CW_USEDEFAULT, width, height,
                             NULL, NULL, NULL, NULL);

    ShowWindow(hwnd, SW_SHOW);

    // timer pour mettre à jour l’état toutes les 200 ms
    // SetTimer(hwnd, 1, 200, NULL);

    MSG msg;
    while (GetMessage(&msg, 0, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}