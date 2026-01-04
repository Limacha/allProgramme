#ifdef _WIN32
#include <windows.h>
#include "gui.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../resource.h"

#define APP_TITLE "minigui - Win32"

typedef struct
{
    int width, height;
    HBITMAP hbmp;
    void *pixels;
} AppState;

static AppState g_app = {800, 480, NULL, NULL};

static BOOL create_offscreen(AppState *s)
{
    BITMAPINFO bmi = {0};
    bmi.bmiHeader.biSize = sizeof(BITMAPINFOHEADER);
    bmi.bmiHeader.biWidth = s->width;
    bmi.bmiHeader.biHeight = -s->height;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    HDC hdc = GetDC(NULL);
    void *bits = NULL;
    HBITMAP hbmp = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &bits, NULL, 0);
    ReleaseDC(NULL, hdc);

    if (!hbmp || !bits)
        return FALSE;

    s->hbmp = hbmp;
    s->pixels = bits;
    return TRUE;
}

static void render(AppState *s)
{
    unsigned int *px = (unsigned int *)s->pixels;
    int w = s->width, h = s->height;

    for (int y = 0; y < h; ++y)
        for (int x = 0; x < w; ++x)
            px[y * w + x] = 0xFFDCDCDC;

    int rx0 = w / 8, ry0 = h / 4, rx1 = w * 7 / 8, ry1 = h * 3 / 4;
    for (int y = ry0; y < ry1; ++y)
        for (int x = rx0; x < rx1; ++x)
            px[y * w + x] = 0xFF0078D7;
}

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    switch (msg)
    {
    case WM_SIZE:
    {
        RECT r;
        GetClientRect(hwnd, &r);
        g_app.width = r.right - r.left;
        g_app.height = r.bottom - r.top;
        if (g_app.hbmp)
            DeleteObject(g_app.hbmp);
        create_offscreen(&g_app);
        return 0;
    }
    case WM_PAINT:
    {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hwnd, &ps);
        render(&g_app);

        HDC mem = CreateCompatibleDC(hdc);
        HBITMAP old = (HBITMAP)SelectObject(mem, g_app.hbmp);
        BitBlt(hdc, 0, 0, g_app.width, g_app.height, mem, 0, 0, SRCCOPY);
        SelectObject(mem, old);
        DeleteDC(mem);
        EndPaint(hwnd, &ps);
        return 0;
    }
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

int gui_main_win32(void)
{
    HINSTANCE hInstance = GetModuleHandle(NULL);

    WNDCLASSA wc = {0};
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = "minigui_class";
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);
    wc.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_APP_ICON)); // icône grande
    // wc.hIconSm = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_APP_ICON)); // icône petite
    RegisterClassA(&wc);

    HWND hwnd = CreateWindowA("minigui_class", APP_TITLE,
                              WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                              CW_USEDEFAULT, CW_USEDEFAULT,
                              g_app.width, g_app.height,
                              NULL, NULL, hInstance, NULL);

    if (!hwnd)
        return 1;
    create_offscreen(&g_app);

    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}
#endif