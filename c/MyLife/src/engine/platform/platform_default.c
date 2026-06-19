#include "platform.h"

static void default_initWindow(const char *t, int w, int h, int fps)
{
    (void)t;
    (void)w;
    (void)h;
    (void)fps;
}
static uint8_t default_windowShouldRun(void) { return 0; }
static void default_beginFrame(void) {}
static void default_update(double d, Input i)
{
    (void)d;
    (void)i;
}
static void default_draw(void) {}
static void default_endFrame(void) {}
static void default_closeWindow(void) {}
static double default_getDelta(void) { return 0.0; }
static int default_getFPS(void) { return 0; }
static Input default_getInput(void)
{
    Input i = {0};
    return i;
}
static void default_limitDelta(double *d) { (void)d; }
static void default_drawRect(int x, int y, int w, int h, Color c)
{
    (void)x;
    (void)y;
    (void)w;
    (void)h;
    (void)c;
}
static void default_drawRectRec(Rect rect, Color c)
{
    (void)rect;
    (void)c;
}
static void default_drawCircle(int cx, int cy, float r, Color c)
{
    (void)cx;
    (void)cy;
    (void)r;
    (void)c;
}
static void default_drawLine(int x1, int y1, int x2, int y2, Color c)
{
    (void)x1;
    (void)y1;
    (void)x2;
    (void)y2;
    (void)c;
}
static void default_drawText(const char *t, int x, int y, int s, Color c)
{
    (void)t;
    (void)x;
    (void)y;
    (void)s;
    (void)c;
}

Platform platform = {
    .initWindow = default_initWindow,
    .windowShouldRun = default_windowShouldRun,
    .beginFrame = default_beginFrame,
    .update = default_update,
    .draw = default_draw,
    .endFrame = default_endFrame,
    .closeWindow = default_closeWindow,
    .getDelta = default_getDelta,
    .getFps = default_getFPS,
    .getInput = default_getInput,
    .limitDelta = default_limitDelta,
    .drawRect = default_drawRect,
    .drawRectRec = default_drawRectRec,
    .drawCircle = default_drawCircle,
    .drawLine = default_drawLine,
    .drawText = default_drawText,
};