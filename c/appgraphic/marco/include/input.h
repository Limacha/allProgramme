#ifndef INPUT_H
#define INPUT_H

typedef enum
{
    INPUT_KEY,
    INPUT_MOUSE_LEFT,
    INPUT_MOUSE_RIGHT
} InputType;

InputType selected_input;
int selected_vk;

// Déclenche un "clic" ou une touche rapidement
void input_click(InputType type);

// Maintenir une touche / bouton
void input_press(InputType type);
void input_release(InputType type);

// Attendre en millisecondes
void input_sleep(unsigned int ms);

#endif
