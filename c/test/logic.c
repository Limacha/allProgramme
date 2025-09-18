#include "logic.h"
#include "input.h"

// --- Paramètres ---
#define MAX_WIDTH 1920
#define MAX_HEIGHT 1080

static unsigned int pixels[MAX_WIDTH * MAX_HEIGHT];
static int width, height;

void initLogic(int w, int h)
{
    if (w > MAX_WIDTH)
        w = MAX_WIDTH;
    if (h > MAX_HEIGHT)
        h = MAX_HEIGHT;
    width = w;
    height = h;

    // Aucun malloc nécessaire, buffer statique déjà alloué
}

void updateLogic(void)
{
    static int frame = 0;
    if (!isSpacePressed()) // mouvement uniquement si espace non pressé
        frame++;

    for (int y = 0; y < height; y++)
    {
        for (int x = 0; x < width; x++)
        {
            pixels[y * width + x] = ((x + frame) << 16) | ((y + frame) << 8);
        }
    }
}

void shutdownLogic(void)
{
    // rien à faire, buffer statique
}

unsigned int *getPixels(void)
{
    return pixels;
}
