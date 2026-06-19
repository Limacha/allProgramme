#ifndef ENGINE_STRUCT_H
#define ENGINE_STRUCT_H

#include <stdint.h> /* uint8_t, uint32_t, int32_t */

/**
 * Couleur RGBA (4 unsigned char)
 */
typedef struct Color
{
    uint8_t r, g, b, a;
} Color;

/**
 * Vecteur 2D (2 int)
 */
typedef struct Vec2i
{
    int32_t x, y;
} Vec2i;

/**
 * Taille (2 int)
 */
typedef struct Size
{
    int32_t width, height;
} Size;

/**
 * rectangle xywh (4 int)
 */
typedef struct Rect
{
    int32_t x, y, width, height;
} Rect;

/**
 * etat des inputs (7 unsigned char + PlatformVec2i)
 */
typedef struct Input
{
    unsigned char key_up;
    unsigned char key_down;
    unsigned char key_left;
    unsigned char key_right;
    unsigned char key_escape;
    unsigned char key_space;
    unsigned char mouse_left; /* bouton gauche pressé */
    Vec2i mouse_pos;          /* position absolue souris */
} Input;

/**
 * an element with xywh (4 uchar)
 */
typedef struct Element
{
    uint8_t x, y, width, height;
} Element;

/**
 * an element with xywh (4 uchar)
 */
typedef struct Button
{
    Element base;
    void (*OnClick)(void);
} Button;

#endif