#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include "../render.h"
#include "../input.h"

static Display *display;
static Window window;
static GC gc;
static XImage *ximage;
static int width, height;
static unsigned int *screen_buffer;

static int space_pressed = 0; // toggle mouvement

// --- INPUT ---
int platformIsSpacePressed(void)
{
    return space_pressed;
}

// --- RENDER ---
void platformInitRender(int w, int h)
{
    width = w;
    height = h;

    // allocation mémoire avec Xlib (pas stdlib.h)
    screen_buffer = (unsigned int *)Xmalloc(sizeof(unsigned int) * w * h);

    display = XOpenDisplay(0);
    if (!display)
        return;

    int screen = DefaultScreen(display);

    window = XCreateSimpleWindow(
        display,
        RootWindow(display, screen),
        0, 0, width, height,
        1,
        BlackPixel(display, screen),
        WhitePixel(display, screen));

    XSelectInput(display, window,
                 ExposureMask | KeyPressMask | StructureNotifyMask);
    XMapWindow(display, window);

    gc = DefaultGC(display, screen);

    ximage = XCreateImage(
        display,
        DefaultVisual(display, screen),
        DefaultDepth(display, screen),
        ZPixmap,
        0,
        (char *)screen_buffer,
        width,
        height,
        32,
        0);
}

void platformRenderFrame(unsigned int *pixels)
{
    // copie manuelle (pas memcpy)
    int n = width * height;
    for (int i = 0; i < n; i++)
    {
        screen_buffer[i] = pixels[i];
    }

    XPutImage(display, window, gc, ximage,
              0, 0, 0, 0,
              width, height);

    XFlush(display);
}

void platformShutdownRender(void)
{
    if (ximage)
    {
        ximage->data = 0; // éviter double free
        XDestroyImage(ximage);
    }

    if (screen_buffer)
        Xfree(screen_buffer);

    if (display)
        XCloseDisplay(display);
}

// --- INPUT ---
void platformInitInput(void)
{
    // Rien à faire ici
}

void platformProcessInput(void)
{
    XEvent event;
    while (XPending(display))
    {
        XNextEvent(display, &event);

        switch (event.type)
        {
        case KeyPress:
        {
            KeySym sym = XLookupKeysym(&event.xkey, 0);
            if (sym == XK_Escape)
                _exit(0); // pas stdlib
            else if (sym == XK_space)
                space_pressed = !space_pressed; // toggle
            break;
        }
        case DestroyNotify:
            _exit(0);
        }
    }
}
