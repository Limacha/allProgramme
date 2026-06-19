#ifndef CONVERTER_H
#define CONVERTER_H

#include "../engineStruct.h"
#include "../raylib_adapter.h"

raylib_Color ColorToRLColor(Color color);
Rectangle RecttoRLRectangle(Rect rect);
raylib_Vec2i Vect2itoRLVector2(Vec2i vector);
Vec2i RLVector2toVect2i(raylib_Vec2i vector);

#define RLCOLOR(color) ColorToRLColor(color)
#define RLRECTANGLE(rect) RecttoRLRectangle(rect)
#define RLVECTOR2(vector) Vect2itoRLVector2(vector)
#define VEC2I(vector) RLVector2toVect2i(vector)
#endif