#ifndef CONTROLLER_H
#define CONTROLLER_H

typedef enum
{
    HOLD,
    CLICK
} input_mode;

extern input_mode selected_mode;
extern unsigned int interval;

// Retourne si le système est en fonctionnement
char controller_is_running();

// Fonction appelée par GUI ou F6
void controller_toggle();

// Démarre la boucle du contrôleur (pour la version console si besoin)
void controller_start();

char controller_nbF6();

void controller_launchThread();
#endif
