#ifndef ENGINE_CONST_H
#define ENGINE_CONST_H

#include "engineStruct.h"

/*
    0.1s  →  10  FPS minimum garanti
    0.05s →  20  FPS minimum
    0.033s → 30  FPS minimum
*/
#define MAX_DELTA_TIME 0.1

// #region COULEURS PREDEFINIS
#define COLOR_BLACK (Color){0, 0, 0, 255}
#define COLOR_WHITE (Color){255, 255, 255, 255}
#define COLOR_RED (Color){255, 0, 0, 255}
#define COLOR_GREEN (Color){0, 200, 0, 255}
#define COLOR_BLUE (Color){0, 0, 255, 255}
#define COLOR_GRAY (Color){128, 128, 128, 255}
#define COLOR_BG (Color){20, 20, 30, 255} /* fond sombre */
// #endregion

#endif