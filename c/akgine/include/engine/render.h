#ifndef RENDER_H
#define RENDER_H

#define DEFAULT_WIDTH 1280
#define DEFAULT_HEIGHT 720

void initRender(unsigned int color);
void updateRender(void);
void closeRender(void);

unsigned int *getPixels(void);

#endif