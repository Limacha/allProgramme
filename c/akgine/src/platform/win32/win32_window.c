#include <windows.h>

#include "../../../include/platform/platform.h"
#include "../../../include/platform/platform_window.h"

static HWND hwnd;                   // handle de la fenetre
static HDC hdc;                     // context pour dessiner
static int width, height;           // taille de la fenetre
static unsigned char running = 1;   // bool si le programme tourne
static unsigned int *screen_buffer; // tableau des pixel

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    // struct abstrait d'un event
    Event event;
    event.type = EVENT_NONE;

    // stock brut si possible (pour debug)
    if (event.rawEvent && sizeof(WPARAM) + sizeof(LPARAM) <= MAX_RAW_EVENT_SIZE)
    {
        // stock brut des valeurs
        *(WPARAM *)(event.rawEvent) = wParam;
        *(LPARAM *)(event.rawEvent + sizeof(WPARAM)) = lParam;
        event.rawSize = sizeof(WPARAM) + sizeof(LPARAM);
    }

    // *(WPARAM *)(event.rawEvent) = wParam;
    // *(LPARAM *)(event.rawEvent + sizeof(WPARAM)) = lParam;
    // event.rawSize = sizeof(WPARAM) + sizeof(LPARAM);

    switch (msg)
    {

    // #region fenetre / app

    // destruction de la fenetre
    case WM_DESTROY:
    {
        event.type = EVENT_QUIT;
        running = 0;
        // termine la boucle de message
        PostQuitMessage(0);
        break;
    }

    // demande de fermeture
    case WM_CLOSE:
    {
        event.type = EVENT_WINDOW_CLOSE;
        break;
    }

    // recoit le focud
    case WM_SETFOCUS:
    {
        event.type = EVENT_WINDOW_FOCUS;
        break;
    }

    // perd le focus
    case WM_KILLFOCUS:
    {
        event.type = EVENT_WINDOW_UNFOCUS;
        break;
    }

    // resize de la fenetre
    case WM_SIZE:
    {
        event.type = EVENT_WINDOW_RESIZE;
        event.window.width = LOWORD(lParam);
        event.window.height = HIWORD(lParam);
        break;
    }

    // commande system
    case WM_SYSCOMMAND:
    {
        if ((wParam & 0xFFF0) == SC_MINIMIZE)
            event.type = EVENT_WINDOW_MINIMIZE;
        else if ((wParam & 0xFFF0) == SC_RESTORE)
            event.type = EVENT_WINDOW_RESTORE;
        break;
    }

    // changement DPI
    case WM_DPICHANGED:
    {
        event.type = EVENT_WINDOW_DPI_CHANGE;
        break;
    }

    // #endregion
    // #region clavier

    // touche presse
    case WM_KEYDOWN:
    case WM_SYSKEYDOWN:
    {
        event.type = EVENT_KEY_DOWN;
        event.key.keycode = (unsigned int)wParam;

        // lecture des modificateurs
        // platformGetModifiers(&event.key.shift, &event.key.ctrl, &event.key.alt, &event.key.win);

        // nb repetition
        event.key.repeat = LOWORD(lParam);
        // touche etendu
        event.key.extended = (lParam & (1 << 24)) ? 1 : 0;
        // touche deja presser
        event.key.pressed_before = (lParam & (1 << 30)) ? 1 : 0;
        break;
    }

    // touche relacher
    case WM_KEYUP:
    case WM_SYSKEYUP:
    {
        event.type = EVENT_KEY_UP;
        event.key.keycode = (unsigned int)wParam;

        // lecture des modificateurs
        // platformGetModifiers(&event.key.shift, &event.key.ctrl, &event.key.alt, &event.key.win);

        // nb repetition
        event.key.repeat = LOWORD(lParam);
        // touche etendu
        event.key.extended = (lParam & (1 << 24)) ? 1 : 0;
        // touche deja presser
        event.key.pressed_before = (lParam & (1 << 30)) ? 1 : 0;
        break;
    }

    // #endregion
    // #region souris

    // mouvement de la souris
    case WM_MOUSEMOVE:
    {
        event.type = EVENT_MOUSE_MOVE;

        goto mouse_pos;
    }

    case WM_LBUTTONDBLCLK:
    {
        event.type = EVENT_MOUSE_DOUBLE_CLICK;
        event.mouse.button = 0;
        goto mouse_pos;
    }

    case WM_LBUTTONDOWN:
    {
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 0;
        goto mouse_pos;
    }
    case WM_LBUTTONUP:
    {
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 0;
        goto mouse_pos;
    }
    case WM_RBUTTONDOWN:
    {
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 1;
        goto mouse_pos;
    }
    case WM_RBUTTONUP:
    {
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 1;
        goto mouse_pos;
    }
    case WM_MBUTTONDOWN:
    {
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 2;
        goto mouse_pos;
    }
    case WM_MBUTTONUP:
    {
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 2;
        goto mouse_pos;
    }
    case WM_XBUTTONDOWN:
    {
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = (HIWORD(wParam) == XBUTTON1 ? 3 : 4);
        goto mouse_pos;
    }
    case WM_XBUTTONUP:
    {
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = (HIWORD(wParam) == XBUTTON1 ? 3 : 4);
        goto mouse_pos;
    }
    case WM_MOUSEWHEEL:
    {
        event.type = EVENT_MOUSE_WHEEL;
        event.mouse.delta = GET_WHEEL_DELTA_WPARAM(wParam);
        goto mouse_pos;
    }

    mouse_pos:
    {
        /*
        // obtient la possition
        event.mouse.x = GET_X_LPARAM(lParam);
        event.mouse.y = GET_Y_LPARAM(lParam);

        // delta depuis la posse d'avant
        event.mouse.dx = event.mouse.x - last_mouse_x;
        event.mouse.dy = event.mouse.y - last_mouse_y;

        // sauvegarde la postion actuel comme la last
        last_mouse_x = event.mouse.x;
        last_mouse_y = event.mouse.y;
*/
        break;
    }

        // #endregion

        /*
            // =======================
            // MULTITOUCH
            // =======================

            // plusieur touche tactiles
            case WM_TOUCH:
            {
                TOUCHINPUT ti[MAX_TOUCH_POINTS];

                // nb de point tactiles
                UINT count = LOWORD(wParam);
                if (count > MAX_TOUCH_POINTS)
                    count = MAX_TOUCH_POINTS;

                if (GetTouchInputInfo((HTOUCHINPUT)lParam, count, ti, sizeof(TOUCHINPUT)))
                {
                    event.type = EVENT_TOUCH;
                    event.touch.count = count;
                    for (UINT i = 0; i < count; i++)
                    {
                        event.touch.points[i].id = ti[i].dwID;
                        event.touch.points[i].x = ti[i].x / 100; // coordonnées en px
                        event.touch.points[i].y = ti[i].y / 100;
                    }

                    // libere la resource
                    CloseTouchInputHandle((HTOUCHINPUT)lParam);
                }
                break;
            }

            // =======================
            // POINTER API
            // =======================

            // stylet / trackpad / touch avancé
            case WM_POINTERDOWN:
            case WM_POINTERUPDATE:
            case WM_POINTERUP:
            {
                POINTER_INFO pi;
                if (GetPointerInfo(GET_POINTERID_WPARAM(wParam), &pi))
                {
                    event.type = EVENT_POINTER;
                    event.pointer.pointer_id = pi.pointerId;
                    event.pointer.pointer_type = pi.pointerType;
                    event.pointer.x = pi.ptPixelLocation.x;
                    event.pointer.y = pi.ptPixelLocation.y;

                    // Si c'est un stylet, lire les infos supplémentaires
                    if (pi.pointerType == PT_PEN)
                    {
                        POINTER_PEN_INFO penInfo;
                        if (GetPointerPenInfo(pi.pointerId, &penInfo))
                        {
                            event.pointer.pressure = penInfo.pressure;
                            event.pointer.tilt_x = penInfo.tiltX;
                            event.pointer.tilt_y = penInfo.tiltY;
                            event.pointer.rotation = penInfo.rotation;
                        }
                    }
                }
                break;
            }
        */

    // message pas traiter
    default:
    {
        event.type = EVENT_RAW;
        break;
    }
    }

    // envoie le message
    // platformPushEvent(event);

    // appel par default
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

void platformInitWindow(char *windowName, int w, int h)
{
    // stock la taille et genere un tableau
    width = w;
    height = h;
    screen_buffer = (unsigned int *)malloc(sizeof(unsigned int) * width * height);

    // Récupère le handle de l'instance du programme courant
    HINSTANCE hInstance = GetModuleHandle(NULL);
    // Déclare une structure de class de fenêtre Win32
    WNDCLASS wc = {0};
    // Associe la fonction de traitement des messages Windows
    wc.lpfnWndProc = WndProc;
    // Associe la class à l'instance de l'application
    wc.hInstance = hInstance;
    // Nom de la class
    wc.lpszClassName = windowName;
    // Enregistre la class
    RegisterClass(&wc);

    hwnd = CreateWindowEx(
        0,                                // Styles étendus (aucun ici)
        windowName,                       // Nom de la classe utiliser
        windowName,                       // Titre de la fenêtre
        WS_OVERLAPPEDWINDOW | WS_VISIBLE, // Fenêtre standard visible
        CW_USEDEFAULT,                    // Position X par défaut
        CW_USEDEFAULT,                    // Position Y par défaut
        width,                            // Largeur de la fenêtre
        height,                           // Hauteur de la fenêtre
        NULL,                             // Pas de fenêtre parente
        NULL,                             // Pas de menu
        hInstance,                        // Instance de l'application
        NULL                              // Paramètre utilisateur (non utilisé)
    );

    // Récupère le Device Context
    // permet de dessiner dans la fenetre
    hdc = GetDC(hwnd);
}

void platformPollEventWindow(unsigned int *pixels)
{
    MSG msg;

    // Traite tous les messages en attente sans bloquer
    while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

void platformCloseWindow(void)
{
    // libere l'espace memoire de l'affichage
    if (screen_buffer)
        free(screen_buffer);
}

unsigned char platformIsRunning(void)
{
    return running;
}