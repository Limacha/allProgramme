#ifndef SPRITE_H
#define SPRITE_H

/**
 * Sprite runtime 2D.
 *
 * pixels (RGBA 32 bits)
 */
typedef struct Sprite
{
    int x; // Position X à l’écran
    int y; // Position Y à l’écran

    int width;  // Largeur du sprite
    int height; // Hauteur du sprite

    const int *pixels; // Pixels RGBA
} Sprite;

/**
 * Initialise un sprite à partir de pixels déjà chargés.
 *
 * @param sprite Sprite à initialiser
 * @param pixels Buffer RGBA (width * height)
 * @param w Largeur
 * @param h Hauteur
 */
void sprite_init(Sprite *sprite,
                 const int *pixels,
                 int w,
                 int h);

#endif