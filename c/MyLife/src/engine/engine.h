#ifndef ENGINE_H
#define ENGINE_H

#include "engineStruct.h"
#include "engineConst.h"

void InitWindow(const char *title, int width, int height, int targetFPS);
uint8_t WindowShouldRun(void);
void BeginFrame(void);
void Update(double delta, Input input);
void Draw(void);
void EndFrame(void);
void CloseWindow(void);

double GetDelta(void);
int GetFPS(void);
Input GetInput(void);

void LimitDeltaTime(double *deltaTime);

uint8_t IsKeyDown(int key);
uint8_t IsKeyPressed(int key);
uint8_t IsMouseButtonDown(int key);
Vec2i GetMousePosition(void);

uint8_t CheckRectCollideToPoint(Vec2i point, Rect rectangle);

void DrawRect(int x, int y, int w, int h, Color color);
void DrawRectRec(Rect rect, Color color);
void DrawCircle(int cx, int cy, float radius, Color color);
void DrawLine(int x1, int y1, int x2, int y2, Color color);
void DrawText(const char *text, int x, int y, int size, Color color);

#endif
