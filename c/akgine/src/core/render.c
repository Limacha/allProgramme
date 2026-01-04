#include "../../include/platform/platform_render.h"
#include "../../include/engine/render.h"

void initRender(unsigned int color) { paltformInitRender(color); }
void updateRender(void) { platformUpdateRender(); }
void closeRender(void) { platformCloseRender(); }

unsigned int *getPixels(void) {}