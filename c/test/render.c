#include "render.h"
#include "platform/platform.h"

void initRender(int w, int h)
{
    platformInitRender(w, h);
}

void renderFrame(unsigned int *pixels)
{
    platformRenderFrame(pixels);
}

void shutdownRender(void)
{
    platformShutdownRender();
}
