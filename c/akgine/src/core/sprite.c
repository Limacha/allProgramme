#include "../../include/engine/sprite.h"

void sprite_init(Sprite *sprite,
                 const int *pixels,
                 int w,
                 int h)
{
    sprite->x = 0;
    sprite->y = 0;
    sprite->width = w;
    sprite->height = h;
    sprite->pixels = pixels;
}
