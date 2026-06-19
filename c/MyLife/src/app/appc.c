#include "app.h"

#include "../engine/engine.h"

#include <math.h>
#include <stdio.h> /* sprintf pour le texte de debug */

/* global value */
static AppState state;

/**
 * init the default value
 */
void app_init(void)
{
    InitWindow(APP_TITLE, APP_WIDTH, APP_HEIGHT, APP_FPS);
    /* player at the center */
    state.player_x = APP_WIDTH / 2.0f - 25.0f;
    state.player_y = APP_HEIGHT / 2.0f - 25.0f;
    state.vel_x = 0.0f;
    state.vel_y = 0.0f;

    /* ball up left with init speed */
    state.ball_x = APP_WIDTH / 2.0f - 25.0f;
    state.ball_y = APP_HEIGHT / 2.0f - 25.0f;
    state.ball_vx = 250.0f; /* pixels/seconde */
    state.ball_vy = 180.0f;
    state.ball_r = 18.0f;

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

    // /* --- Déplacement du joueur ---
    //    On applique une accélération selon les touches,
    //    puis on freine légèrement chaque frame (friction). */
    // const float ACCEL = 600.0f;
    // const float FRICTION = 0.85f; /* facteur multiplicatif */

    // if (input.key_left)
    //    state.vel_x -= ACCEL * (float)delta;
    // if (input.key_right)
    //    state.vel_x += ACCEL * (float)delta;
    // if (input.key_up)
    //    state.vel_y -= ACCEL * (float)delta;
    // if (input.key_down)
    //    state.vel_y += ACCEL * (float)delta;

    // /* Application de la friction */
    // state.vel_x *= FRICTION;
    // state.vel_y *= FRICTION;

    // /* Intégration de la position */
    // state.player_x += state.vel_x * (float)delta;
    // state.player_y += state.vel_y * (float)delta;

    // /* Garder le joueur dans l'écran */
    // if (state.player_x < 0)
    //    state.player_x = 0;
    // if (state.player_y < 0)
    //    state.player_y = 0;
    // if (state.player_x > APP_WIDTH - 50)
    //    state.player_x = APP_WIDTH - 50;
    // if (state.player_y > APP_HEIGHT - 50)
    //    state.player_y = APP_HEIGHT - 50;

    // /* --- Balle rebondissante ---
    //    Physique simple : rebond sur les bords par inversion de vitesse */
    // state.ball_x += state.ball_vx * (float)delta;
    // state.ball_y += state.ball_vy * (float)delta;

    // /* Rebond horizontal */
    // if (state.ball_x - state.ball_r < 0)
    // {
    //    state.ball_x = state.ball_r;
    //    state.ball_vx = -state.ball_vx;
    // }
    // if (state.ball_x + state.ball_r > APP_WIDTH)
    // {
    //    state.ball_x = APP_WIDTH - state.ball_r;
    //    state.ball_vx = -state.ball_vx;
    // }

    // /* Rebond vertical */
    // if (state.ball_y - state.ball_r < 0)
    // {
    //    state.ball_y = state.ball_r;
    //    state.ball_vy = -state.ball_vy;
    // }
    // if (state.ball_y + state.ball_r > APP_HEIGHT)
    // {
    //    state.ball_y = APP_HEIGHT - state.ball_r;
    //    state.ball_vy = -state.ball_vy;
    // }

    /* Mise à jour du texte de statut */
    /*sprintf(state.status, "FPS [%3d]  |  t=%5.2fs d=%5.2lf  |  Balle: (%5.2f, %10.2f)  |  Mouse(%5d, %10d) |  Joueur: (%5.2f, %5.2f)",
            GetFPS(),
            state.time, delta,
            state.ball_x, state.ball_y,
            input.mouse_pos.x, input.mouse_pos.y,
            state.player_x, state.player_y);*/
    // sprintf(state.status, "FPS [%3d]  |  Mouse(%5d, %10d) click:%c |  JoueurVel: (%5.2f, %5.2f)  |  t=%5.2fs d=%5.2lf",
    //         GetFPS(),
    //         input.mouse_pos.x, input.mouse_pos.y, (input.mouse_left ? 'T' : 'F'),
    //         state.vel_x, state.vel_y,
    //         state.time, delta);
    sprintf(state.status, "FPS [%3d]  |  Mouse(%5d, %10d) click:%c |  t=%5.2fs d=%5.2lf",
            GetFPS(),
            input.mouse_pos.x, input.mouse_pos.y, (input.mouse_left ? 'T' : 'F'),
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

    // /* --- Effet de pulsation sur le joueur ---
    //    Le vert varie légèrement dans le temps grâce à un sinus.
    //    sinf retourne [-1, 1], on l'adapte à [180, 255]. */
    // unsigned char pulse = (unsigned char)(200 + 55 * sinf((float)state.time * 3.0f));
    // Color player_color = {0, pulse, 80, 255};

    // /* Joueur : carré 50x50 avec une bordure blanche */
    // DrawRect((int)state.player_x - 2,
    //          (int)state.player_y - 2, 54, 54, COLOR_WHITE);
    // DrawRect((int)state.player_x,
    //          (int)state.player_y, 50, 50, player_color);

    // /* --- Balle ---
    //    Couleur qui change avec le temps (cycle rouge → bleu) */
    // unsigned char r = (unsigned char)(128 + 127 * sinf((float)state.time * 2.0f));
    // unsigned char b = (unsigned char)(128 + 127 * cosf((float)state.time * 2.0f));
    // Color ball_color = {r, 100, b, 255};
    // DrawCircle((int)state.ball_x, (int)state.ball_y,
    //            (int)state.ball_r, ball_color);

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