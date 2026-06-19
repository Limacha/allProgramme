#ifndef RAYLIB_ADAPTER_H
#define RAYLIB_ADAPTER_H

// on wrap les fonctions de raylib
#define InitWindow raylib_InitWindow
#define WindowShouldClose raylib_WindowShouldClose
#define CloseWindow raylib_CloseWindow
#define GetFPS raylib_GetFPS

#define IsKeyDown raylib_IsKeyDown
#define IsKeyPressed raylib_IsKeyPressed
#define IsMouseButtonDown raylib_IsMouseButtonDown
#define GetMousePosition raylib_GetMousePosition

#define DrawCircle raylib_DrawCircle
#define DrawLine raylib_DrawLine
#define DrawText raylib_DrawText

#define Color raylib_Color
#define Vector2 raylib_Vec2i
// #define Size raylib_Size
// #define Input raylib_Input

// #define COLOR_BLACK raylib_COLOR_BLACK
// #define COLOR_WHITE raylib_COLOR_WHITE
// #define COLOR_RED raylib_COLOR_RED
// #define COLOR_GREEN raylib_COLOR_GREEN
// #define COLOR_BLUE raylib_COLOR_BLUE
// #define COLOR_GRAY raylib_COLOR_GRAY
// #define COLOR_BLACK raylib_COLOR_BLACK

#include <raylib.h>

// on restore le nom pour la suite
#undef InitWindow
#undef WindowShouldClose
#undef CloseWindow
#undef GetFPS

#undef IsKeyDown
#undef IsKeyPressed
#undef IsMouseButtonDown
#undef GetMousePosition

#undef DrawCircle
#undef DrawLine
#undef DrawText

#undef Color
#undef Vec2i
// #undef Size
// #undef Input

// #undef COLOR_BLACK
// #undef COLOR_WHITE
// #undef COLOR_RED
// #undef COLOR_GREEN
// #undef COLOR_BLUE
// #undef COLOR_GRAY

#endif