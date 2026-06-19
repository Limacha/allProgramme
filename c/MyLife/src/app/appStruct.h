#ifndef APP_STRUCT_H
#define APP_STRUCT_H

#include <stdint.h> /* uint8_t, uint32_t, int32_t */

/* ---------------------------------------------------------
   ÉTAT DE L'APPLICATION
   Toutes les variables qui représentent l'"état vivant"
   de l'app sont regroupées dans un struct. C'est bien
   meilleur que des globales éparpillées : tout est lisible
   et transmissible (ex. pour une sauvegarde).
   --------------------------------------------------------- */
typedef struct AppState
{
    /* Position et vitesse du carré contrôlable */
    // float player_x, player_y;
    // float vel_x, vel_y;

    // /* Balle rebondissante (démonstration de la physique simple) */
    // float ball_x, ball_y;
    // float ball_vx, ball_vy;
    // float ball_r; /* rayon */

    /* Compteur de frames pour les animations */
    double time;

    /* Texte de statut affiché en bas */
    char status[128];
} AppState;

#endif