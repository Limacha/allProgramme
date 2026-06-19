#ifndef PLATFORM_H
#define PLATFORM_H

#include <stdint.h>
#include "../engineStruct.h"

// Pointeurs de fonctions pour chaque plateforme
typedef void (*platform_init_fn)(const char *title, int width, int height, int targetFPS);
typedef uint8_t (*platform_window_should_run_fn)(void);
typedef void (*platform_begin_frame_fn)(void);
typedef void (*platform_update_fn)(double delta, Input input);
typedef void (*platform_draw_fn)(void);
typedef void (*platform_end_frame_fn)(void);
typedef void (*platform_close_fn)(void);
typedef double (*platform_get_delta_fn)(void);
typedef int (*platform_get_fps_fn)(void);
typedef Input (*platform_get_input_fn)(void);
typedef void (*platform_limit_delta_fn)(double *deltaTime);
typedef void (*platform_draw_rect_fn)(int x, int y, int width, int height, Color color);
typedef void (*platform_draw_rect_rec_fn)(Rect rect, Color color);
typedef void (*platform_draw_circle_fn)(int cx, int cy, float radius, Color color);
typedef void (*platform_draw_line_fn)(int x1, int y1, int x2, int y2, Color color);
typedef void (*platform_draw_text_fn)(const char *text, int x, int y, int size, Color color);

typedef struct
{
    platform_init_fn initWindow;
    platform_window_should_run_fn windowShouldRun;
    platform_begin_frame_fn beginFrame;
    platform_update_fn update;
    platform_draw_fn draw;
    platform_end_frame_fn endFrame;
    platform_close_fn closeWindow;
    platform_get_delta_fn getDelta;
    platform_get_fps_fn getFps;
    platform_get_input_fn getInput;
    platform_limit_delta_fn limitDelta;
    platform_draw_rect_fn drawRect;
    platform_draw_rect_rec_fn drawRectRec;
    platform_draw_circle_fn drawCircle;
    platform_draw_line_fn drawLine;
    platform_draw_text_fn drawText;
} Platform;

// Implémentation vide par défaut
extern Platform platform;

#endif