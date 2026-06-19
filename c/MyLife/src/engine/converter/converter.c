#include "converter.h"

raylib_Color ColorToRLColor(Color color)
{
    raylib_Color result;
    result.r = color.r;
    result.g = color.g;
    result.b = color.b;
    result.a = color.b;
    return result;
}

Rectangle RecttoRLRectangle(Rect rect)
{
    Rectangle result;
    result.x = rect.x;
    result.y = rect.y;
    result.width = rect.width;
    result.height = rect.height;
    return result;
}

raylib_Vec2i Vect2itoRLVector2(Vec2i vector)
{
    raylib_Vec2i result;
    result.x = vector.x;
    result.y = vector.y;
    return result;
}

Vec2i RLVector2toVect2i(raylib_Vec2i vector)
{
    Vec2i result;
    result.x = vector.x;
    result.y = vector.y;
    return result;
}