#include "engine.h"
#include "raylib_adapter.h"
#include "converter/converter.h"
#include "platform/platform.h"

static double lastTime = 0.0;
static double deltaTime = 0.0;
static Input input = {0};

void InitWindow(const char *title, int width, int height, int targetFPS)
{
    SetTargetFPS(targetFPS);

    raylib_InitWindow(width, height, title);

    lastTime = GetTime();

    platform.initWindow(title, width, height, targetFPS);
}

uint8_t WindowShouldRun(void)
{
    return !raylib_WindowShouldClose();
}

void BeginFrame(void)
{

    /* set time value */
    double now = GetTime();
    deltaTime = now - lastTime;
    lastTime = now;

    /* if delta to high -> game slow insted of freeze for avoid teleporting items and don't have collision apply */
    LimitDeltaTime(&deltaTime);

    // norm pas de crash sur android
    /* get inputs state and mouse position */
    input.key_up = IsKeyDown(KEY_UP) || IsKeyDown(KEY_W);
    input.key_down = IsKeyDown(KEY_DOWN) || IsKeyDown(KEY_S);
    input.key_left = IsKeyDown(KEY_LEFT) || IsKeyDown(KEY_A);
    input.key_right = IsKeyDown(KEY_RIGHT) || IsKeyDown(KEY_D);
    input.key_escape = IsKeyPressed(KEY_ESCAPE);
    input.key_space = IsKeyPressed(KEY_SPACE);
    input.mouse_left = IsMouseButtonDown(MOUSE_BUTTON_LEFT);

    Vec2i mp = GetMousePosition();
    input.mouse_pos.x = (int32_t)mp.x;
    input.mouse_pos.y = (int32_t)mp.y;

    /* initWindow the space for drawing */
    BeginDrawing();

    platform.beginFrame();
}

void Update(double delta, Input input)
{
    platform.update(delta, input);
}

void Draw(void)
{
    platform.draw();
}

void EndFrame(void)
{
    EndDrawing();

    platform.endFrame();
}

void CloseWindow(void)
{
    raylib_CloseWindow();

    platform.closeWindow();
}

// need to implement platform
double GetDelta(void)
{
    return deltaTime;
}

// need to implement platform
int GetFPS(void)
{
    return raylib_GetFPS();
}

// need to implement platform
Input GetInput(void)
{
    return input;
}

// need to implement platform
void LimitDeltaTime(double *deltaTime)
{
    if (*deltaTime > MAX_DELTA_TIME)
        *deltaTime = MAX_DELTA_TIME;
}

uint8_t IsKeyDown(int key)
{
    return raylib_IsKeyDown(key);
}

uint8_t IsKeyPressed(int key)
{
    return raylib_IsKeyPressed(key);
}

uint8_t IsMouseButtonDown(int key)
{
    return raylib_IsMouseButtonDown(key);
}

Vec2i GetMousePosition(void)
{
    return VEC2I(raylib_GetMousePosition());
}

uint8_t CheckRectCollideToPoint(Vec2i point, Rect rectangle)
{
    return CheckCollisionPointRec(RLVECTOR2(point), RLRECTANGLE(rectangle));
}

void DrawRect(int x, int y, int w, int h, Color color)
{
    DrawRectangle(x, y, w, h, RLCOLOR(color));
    platform.drawRect(x, y, w, h, color);
}

void DrawRectRec(Rect rect, Color color)
{
    DrawRectangleRec(RLRECTANGLE(rect), RLCOLOR(color));
    platform.drawRectRec(rect, color);
}

void DrawCircle(int cx, int cy, float radius, Color color)
{
    raylib_DrawCircle(cx, cy, radius, RLCOLOR(color));
    platform.drawCircle(cx, cy, radius, color);
}

void DrawLine(int x1, int y1, int x2, int y2, Color color)
{
    raylib_DrawLine(x1, y1, x2, y2, RLCOLOR(color));
    platform.drawLine(x1, y1, x2, y2, color);
}

void DrawText(const char *text, int x, int y, int size, Color color)
{
    raylib_DrawText(text, x, y, size, RLCOLOR(color));
    platform.drawText(text, x, y, size, color);
}
