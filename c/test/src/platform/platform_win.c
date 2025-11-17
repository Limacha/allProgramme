#include <windows.h>
#include <windowsx.h>
#include "platform.h"
#include "../render.h"
#include "../input.h"

static HWND hwnd; // handle de la fenetre
static HDC hdc;   // context pour dessiner
static int width, height;
static unsigned int *screen_buffer;     // tableau des pixel
static unsigned char space_pressed = 0; // boolean de la touche space
static unsigned char running = 1;       // bool si le programme tourne
static CRITICAL_SECTION queue_lock;

/**
 * @brief fonction appeller a chaque message pour la fenetre
 *
 * @param hwnd la fenetre
 * @param msg le type du message
 * @param wParam info sup
 * @param lParam info sup aussi
 */

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    Event event;
    event.type = EVENT_NONE;
    if (event.rawEvent && sizeof(WPARAM) + sizeof(LPARAM) <= MAX_RAW_EVENT_SIZE)
    {
        // copier les valeurs
        *(WPARAM *)(event.rawEvent) = wParam;
        *(LPARAM *)(event.rawEvent + sizeof(WPARAM)) = lParam;
        event.rawSize = sizeof(WPARAM) + sizeof(LPARAM);
    }
    // *(WPARAM *)(event.rawEvent) = wParam;
    // *(LPARAM *)(event.rawEvent + sizeof(WPARAM)) = lParam;
    // event.rawSize = sizeof(WPARAM) + sizeof(LPARAM);

    switch (msg)
    {

    // ----------- FERMETURE & FOCUS
    case WM_DESTROY:
        event.type = EVENT_QUIT;
        running = 0;
        PostQuitMessage(0);
        break;

    case WM_CLOSE:
        event.type = EVENT_WINDOW_CLOSE;
        break;

    case WM_SETFOCUS:
        event.type = EVENT_WINDOW_FOCUS;
        break;

    case WM_KILLFOCUS:
        event.type = EVENT_WINDOW_UNFOCUS;
        break;

    case WM_SIZE:
        event.type = EVENT_WINDOW_RESIZE;
        event.window.width = LOWORD(lParam);
        event.window.height = HIWORD(lParam);
        break;

    case WM_SYSCOMMAND:
        if ((wParam & 0xFFF0) == SC_MINIMIZE)
            event.type = EVENT_WINDOW_MINIMIZE;
        else if ((wParam & 0xFFF0) == SC_RESTORE)
            event.type = EVENT_WINDOW_RESTORE;
        break;

    case WM_DPICHANGED:
        event.type = EVENT_WINDOW_DPI_CHANGE;
        break;

    // ----------- CLAVIER
    case WM_KEYDOWN:
    case WM_SYSKEYDOWN:
        event.type = EVENT_KEY_DOWN;
        event.key.keycode = (unsigned int)wParam;
        platformGetModifiers(&event.key.shift, &event.key.ctrl, &event.key.alt, &event.key.win);
        event.key.repeat = LOWORD(lParam);
        event.key.extended = (lParam & (1 << 24)) ? 1 : 0;
        event.key.pressed_before = (lParam & (1 << 30)) ? 1 : 0;
        break;

    case WM_KEYUP:
    case WM_SYSKEYUP:
        event.type = EVENT_KEY_UP;
        event.key.keycode = (unsigned int)wParam;
        platformGetModifiers(&event.key.shift, &event.key.ctrl, &event.key.alt, &event.key.win);
        event.key.repeat = LOWORD(lParam);
        event.key.extended = (lParam & (1 << 24)) ? 1 : 0;
        event.key.pressed_before = (lParam & (1 << 30)) ? 1 : 0;
        break;

    // ----------- SOURIS
    case WM_MOUSEMOVE:
        event.type = EVENT_MOUSE_MOVE;
        event.mouse.x = GET_X_LPARAM(lParam);
        event.mouse.y = GET_Y_LPARAM(lParam);
        event.mouse.dx = event.mouse.x - last_mouse_x;
        event.mouse.dy = event.mouse.y - last_mouse_y;
        last_mouse_x = event.mouse.x;
        last_mouse_y = event.mouse.y;
        break;

    case WM_LBUTTONDBLCLK:
        event.type = EVENT_MOUSE_DOUBLE_CLICK;
        event.mouse.button = 0;
        goto mouse_pos;

    case WM_LBUTTONDOWN:
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 0;
        goto mouse_pos;
    case WM_LBUTTONUP:
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 0;
        goto mouse_pos;
    case WM_RBUTTONDOWN:
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 1;
        goto mouse_pos;
    case WM_RBUTTONUP:
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 1;
        goto mouse_pos;
    case WM_MBUTTONDOWN:
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = 2;
        goto mouse_pos;
    case WM_MBUTTONUP:
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = 2;
        goto mouse_pos;
    case WM_XBUTTONDOWN:
        event.type = EVENT_MOUSE_BUTTON_DOWN;
        event.mouse.button = (HIWORD(wParam) == XBUTTON1 ? 3 : 4);
        goto mouse_pos;
    case WM_XBUTTONUP:
        event.type = EVENT_MOUSE_BUTTON_UP;
        event.mouse.button = (HIWORD(wParam) == XBUTTON1 ? 3 : 4);
        goto mouse_pos;
    case WM_MOUSEWHEEL:
        event.type = EVENT_MOUSE_WHEEL;
        event.mouse.delta = GET_WHEEL_DELTA_WPARAM(wParam);
        goto mouse_pos;

    mouse_pos:
        event.mouse.x = GET_X_LPARAM(lParam);
        event.mouse.y = GET_Y_LPARAM(lParam);
        event.mouse.dx = event.mouse.x - last_mouse_x;
        event.mouse.dy = event.mouse.y - last_mouse_y;
        last_mouse_x = event.mouse.x;
        last_mouse_y = event.mouse.y;
        break;

    // ----------- MULTITOUCH (WM_TOUCH)
    case WM_TOUCH:
    {
        TOUCHINPUT ti[MAX_TOUCH_POINTS];
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
            CloseTouchInputHandle((HTOUCHINPUT)lParam);
        }
        break;
    }

    // ----------- WM_POINTER (Stylet / Trackpad / Touch avancé)
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

    default:
        event.type = EVENT_RAW;
        break;
    }

    // platformPushEvent(event);
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

static void platformGetModifiers(unsigned char *shift, unsigned char *ctrl, unsigned char *alt, unsigned char *win)
{
    *shift = (GetKeyState(VK_SHIFT) & 0x8000) ? 1 : 0;
    *ctrl = (GetKeyState(VK_CONTROL) & 0x8000) ? 1 : 0;
    *alt = (GetKeyState(VK_MENU) & 0x8000) ? 1 : 0;
    *win = (GetKeyState(VK_LWIN) & 0x8000 || GetKeyState(VK_RWIN) & 0x8000) ? 1 : 0;
}

/**
 * @brief savoir si l'app tourne encore
 */
unsigned char platformIsRunning(void)
{
    return running;
}

/**
 * @brief
 *
 * @param w la largeur de la fenetre
 * @param h la hauteur de la fenetre
 */
void platformInitRender(int w, int h)
{
    width = w;
    height = h;
    screen_buffer = (unsigned int *)malloc(sizeof(unsigned int) * w * h);

    HINSTANCE hInstance = GetModuleHandle(NULL);
    WNDCLASS wc = {0};
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = "PixelWindow";
    RegisterClass(&wc);

    hwnd = CreateWindowEx(
        0, "PixelWindow", "Pixel Window",
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT, CW_USEDEFAULT, width, height,
        NULL, NULL, hInstance, NULL);

    hdc = GetDC(hwnd);
}

void platformRenderFrame(unsigned int *pixels)
{
    // Copier pixels dans buffer interne
    memcpy(screen_buffer, pixels, sizeof(unsigned int) * width * height);

    BITMAPINFO bmi = {0};
    bmi.bmiHeader.biSize = sizeof(BITMAPINFOHEADER);
    bmi.bmiHeader.biWidth = width;
    bmi.bmiHeader.biHeight = -height; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    StretchDIBits(
        hdc,
        0, 0, width, height,
        0, 0, width, height,
        screen_buffer,
        &bmi,
        DIB_RGB_COLORS,
        SRCCOPY);
}

void platformShutdownRender(void)
{
    if (screen_buffer)
        free(screen_buffer);
}

void platformInitInput(void)
{
    // Rien à faire pour Windows
}

void platformProcessInput(void)
{
    MSG msg;
    while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

// Expose l’état pour input.c
int platformIsSpacePressed(void)
{
    return space_pressed;
}

// #region gestion fichier/dossier

unsigned short platformGetCurrentPath(char *path)
{
    return GetCurrentDirectoryA(MAX_PATH_LENGHT, path);
}

unsigned char platformWriteFile(const char *filename, const char *data, unsigned long size)
{
    HANDLE hFile;
    DWORD bytesWritten;

    hFile = CreateFileA(
        filename,              // nom du fichier (ANSI)
        GENERIC_WRITE,         // accès en écriture
        0,                     // pas de partage
        NULL,                  // sécurité par défaut
        CREATE_ALWAYS,         // crée ou écrase le fichier
        FILE_ATTRIBUTE_NORMAL, // attributs normaux
        NULL                   // pas de modèle
    );

    if (hFile == INVALID_HANDLE_VALUE)
    {
        return FALSE;
    }

    unsigned char result = WriteFile(hFile, data, (DWORD)size, &bytesWritten, NULL);
    CloseHandle(hFile);

    return result;
}

unsigned char platformAppendToFile(const char *filename, const char *data)
{
    HANDLE hFile;
    DWORD bytesWritten;

    hFile = CreateFileA(
        filename,              // nom du fichier
        FILE_APPEND_DATA,      // droit d'ajouter des données
        FILE_SHARE_READ,       // permet la lecture en parallèle
        NULL,                  // sécurité par défaut
        OPEN_ALWAYS,           // ouvre ou crée le fichier
        FILE_ATTRIBUTE_NORMAL, // attributs normaux
        NULL                   // pas de modèle
    );

    if (hFile == INVALID_HANDLE_VALUE)
    {
        return 0; // erreur
    }

    // Déplacer le curseur à la fin (optionnel avec FILE_APPEND_DATA, mais sûr)
    SetFilePointer(hFile, 0, NULL, FILE_END);

    BOOL result = WriteFile(hFile, data, (DWORD)strlen(data), &bytesWritten, NULL);
    CloseHandle(hFile);

    return result ? 1 : 0;
}

void *platformReadFileBinary(const char *path, unsigned long *out_size)
{
    if (!path || !out_size)
        return NULL;

    HANDLE hFile = CreateFileA(
        path,
        GENERIC_READ,
        FILE_SHARE_READ,
        NULL,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        NULL);
    if (hFile == INVALID_HANDLE_VALUE)
        return NULL;

    DWORD size = GetFileSize(hFile, NULL);
    if (size == INVALID_FILE_SIZE || size == 0)
    {
        CloseHandle(hFile);
        *out_size = 0;
        return NULL;
    }

    void *buffer = HeapAlloc(GetProcessHeap(), 0, size);
    if (!buffer)
    {
        CloseHandle(hFile);
        *out_size = 0;
        return NULL;
    }

    DWORD read_bytes = 0;
    if (!ReadFile(hFile, buffer, size, &read_bytes, NULL) || read_bytes != size)
    {
        HeapFree(GetProcessHeap(), 0, buffer);
        CloseHandle(hFile);
        *out_size = 0;
        return NULL;
    }

    CloseHandle(hFile);
    *out_size = (unsigned long)size;
    return buffer;
}

unsigned char platformFileExists(const char *path)
{
    DWORD attrib = GetFileAttributesA(path);
    if (attrib == INVALID_FILE_ATTRIBUTES)
        return 0;                                    // n'existe pas
    return (attrib & FILE_ATTRIBUTE_DIRECTORY) == 0; // vrai si ce n'est PAS un répertoire
}

unsigned char platformDirExists(const char *path)
{
    DWORD attrib = GetFileAttributesA(path);
    if (attrib == INVALID_FILE_ATTRIBUTES)
        return 0;                                    // n'existe pas
    return (attrib & FILE_ATTRIBUTE_DIRECTORY) != 0; // vrai si c'est un dossier
}

unsigned char platformPathExists(const char *path)
{
    DWORD attrib = GetFileAttributesA(path);
    return attrib != INVALID_FILE_ATTRIBUTES; // n'existe pas
}

unsigned char platformCreateDir(const char *path)
{
    if (CreateDirectoryA(path, NULL))
        return 1; // succès
    return 0;     // échec (déjà existant ou erreur)
}

char **platformListDir(const char *path, unsigned int *outCount)
{
    char **result;
    result = (char **)0;
    outCount = 0;

    WIN32_FIND_DATAA findData;
    char searchPath[MAX_PATH];

    // Construire le chemin de recherche : "C:/mon/dossier/*"
    int i = 0;
    while (path[i] && i < (MAX_PATH - 3))
    {
        searchPath[i] = path[i];
        i++;
    }
    searchPath[i++] = '/';
    searchPath[i++] = '*';
    searchPath[i] = '\0';

    HANDLE hFind = FindFirstFileA(searchPath, &findData);
    if (hFind == INVALID_HANDLE_VALUE)
        return result; // retourne vide si erreur

    // Première passe : compter les entrées
    unsigned int count = 0;
    do
    {
        if (findData.cFileName[0] == '.' &&
            (findData.cFileName[1] == '\0' ||
             (findData.cFileName[1] == '.' && findData.cFileName[2] == '\0')))
            continue; // ignorer "." et ".."

        count++;
    } while (FindNextFileA(hFind, &findData));

    FindClose(hFind);

    if (count == 0)
        return result; // aucun fichier -> retourne vide

    // Allouer le tableau de pointeurs
    result = (char **)malloc(count * sizeof(char *));
    if (!result)
        return result;

    // Deuxième passe : stocker les noms
    hFind = FindFirstFileA(searchPath, &findData);
    if (hFind == INVALID_HANDLE_VALUE)
        return result; // improbable mais sécurité

    unsigned int index = 0;
    do
    {
        if (findData.cFileName[0] == '.' &&
            (findData.cFileName[1] == '\0' ||
             (findData.cFileName[1] == '.' && findData.cFileName[2] == '\0')))
            continue;

        // Construire le chemin complet
        int len = 0;
        while (path[len])
            len++;
        int fileLen = 0;
        while (findData.cFileName[fileLen])
            fileLen++;

        int fullLen = len + 1 + fileLen;             // +1 pour '\'
        result[index] = (char *)malloc(fullLen + 1); // +1 pour '\0'

        if (result[index])
        {
            int k = 0;
            for (int j = 0; j < len; j++)
                result[index][k++] = path[j];
            result[index][k++] = '/';
            for (int j = 0; j < fileLen; j++)
                result[index][k++] = findData.cFileName[j];
            result[index][k] = '\0';
        }

        index++;
    } while (FindNextFileA(hFind, &findData));

    FindClose(hFind);
    outCount = count;

    return result;
}

void platformFreeFileBinary(void *buffer)
{
    if (buffer)
        HeapFree(GetProcessHeap(), 0, buffer);
}

// #endregion

// #region gestion event queue

static unsigned char platformPushEvent(Event event)
{
    // lock la queue
    EnterCriticalSection(&queue_lock);
    int next = (eventQueue.end + 1) % MAX_EVENTS; // si max reviens a 0
    if (next == eventQueue.start)
    {
        // queue pleine : on decale le debut
        eventQueue.start = (eventQueue.start + 1) % MAX_EVENTS; // si max reviens a 0
    }
    // ecrit l'event et set le dernier numero
    eventQueue.queue[eventQueue.end] = event;
    eventQueue.end = next;
    // delock la queue
    LeaveCriticalSection(&queue_lock);
}

unsigned char platformGetEvent(Event *out)
{
    // lock la queue
    EnterCriticalSection(&queue_lock);
    if (eventQueue.start == eventQueue.end)
    {
        LeaveCriticalSection(&queue_lock);
        return 0;
    }
    *out = eventQueue.queue[eventQueue.start];
    eventQueue.start = (eventQueue.start + 1) % MAX_EVENTS;
    // delock la queue
    LeaveCriticalSection(&queue_lock);
    return 1;
}

void platformInitEventQueue()
{
    // init le system de lockage
    InitializeCriticalSection(&queue_lock);
}
// #endregion

// #region memory

void *platformMemoryAlloc(unsigned int size)
{
    return malloc(size); // renvoie un pointeur ou NULL si erreur
}

void platformMemoryFree(void *ptr)
{
    free(ptr); // libère le bloc
}
// #endregion

// #region date time

void platformGetDate(unsigned short *year, unsigned short *month, unsigned short *day)
{
    SYSTEMTIME st;
    GetLocalTime(&st); // Récupère la date/heure locale
    if (year)
        *year = st.wYear;
    if (month)
        *month = st.wMonth;
    if (day)
        *day = st.wDay;
}

void platformGetTime(unsigned short *hour, unsigned short *minute, unsigned short *second)
{
    SYSTEMTIME st;
    GetLocalTime(&st); // Récupère la date/heure locale
    if (hour)
        *hour = st.wHour;
    if (minute)
        *minute = st.wMinute;
    if (second)
        *second = st.wSecond;
}
// #endregion
