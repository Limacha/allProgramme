#include "app.h"

#include "../engine/engine.h"

#include <math.h>
#include <stdio.h> /* sprintf pour le texte de debug */

/* global value */
static AppState state;
Rect btnRect = {50, 50, 150, 150};

/**
 * init the default value
 */
void app_init(void)
{
   InitWindow(APP_TITLE, APP_WIDTH, APP_HEIGHT, APP_FPS);

   state.time = 0.0;
}

uint8_t app_should_run(void)
{
   return WindowShouldRun();
}

void app_begin_frame(void)
{
   BeginFrame();
}

/**
 * call each frame
 * @param delta - time from the last frame (0.016 -> 60fps)
 * @param input - all input state
 */
void app_update()
{
   double delta = GetDelta();
   Input input = GetInput();

   state.time += delta;

   /* Mise à jour du texte de statut */
   sprintf(state.status, "FPS [%3d]  |  Mouse(%5d, %10d) click:%c btn:%c |  t=%5.2fs d=%5.2lf",
           GetFPS(),
           input.mouse_pos.x, input.mouse_pos.y, (input.mouse_left ? 'T' : 'F'), (CheckRectCollideToPoint(input.mouse_pos, btnRect) ? 'T' : 'F'),
           state.time, delta);
}

/**
 * draw the app content
 */
void app_draw(void)
{
   /* --- Fond ---
      On dessine un rectangle qui couvre tout l'écran.
      C'est le "clear" : efface ce qui était avant. */
   DrawRect(0, 0, APP_WIDTH, APP_HEIGHT, COLOR_BG);

   /* --- Grille de fond (effet visuel, optionnel) ---
      Lignes tous les 50px en gris très sombre */
   Color grid_color = {35, 35, 50, 255};
   for (int x = 0; x < APP_WIDTH; x += 50)
      DrawLine(x, 0, x, APP_HEIGHT, grid_color);
   for (int y = 0; y < APP_HEIGHT; y += 50)
      DrawLine(0, y, APP_WIDTH, y, grid_color);

   DrawRectRec(btnRect, COLOR_BLUE);

   /* --- UI : barre du haut ---
      Rectangle semi-transparent pour le titre */
   Color bar_color = {10, 10, 20, 200};
   DrawRect(0, 0, APP_WIDTH, 36, bar_color);
   DrawText(APP_TITLE, 10, 8, 20, COLOR_WHITE);

   /* Texte de statut en bas */
   DrawRect(0, APP_HEIGHT - 30, APP_WIDTH, 30, bar_color);
   DrawText(state.status, 10, APP_HEIGHT - 22, 16, COLOR_GRAY);

   /* Instructions en haut à droite */
   DrawText("[ESC] Quitter", APP_WIDTH - 150, 8, 16, COLOR_GRAY);
}

void app_end_frame(void)
{
   EndFrame();
}

void app_close(void)
{
   CloseWindow();
}