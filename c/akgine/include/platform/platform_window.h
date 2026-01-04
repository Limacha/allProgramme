#ifndef PLATFORM_WINDOW_H
#define PLATFORM_WINDOW_H

#define MAX_EVENTS 256
#define MAX_TOUCH_POINTS 16 // nombre max de contacts multitouch simultanés
#define MAX_RAW_EVENT_SIZE 32

/// @brief enum des types d'event
typedef enum
{
    EVENT_NONE,
    EVENT_QUIT,
    EVENT_KEY_DOWN,
    EVENT_KEY_UP,
    EVENT_MOUSE_MOVE,
    EVENT_MOUSE_BUTTON_DOWN,
    EVENT_MOUSE_BUTTON_UP,
    EVENT_MOUSE_WHEEL,
    EVENT_MOUSE_RAW_MOVE,
    EVENT_MOUSE_DOUBLE_CLICK,
    EVENT_WINDOW_RESIZE,
    EVENT_WINDOW_CLOSE,
    EVENT_WINDOW_FOCUS,
    EVENT_WINDOW_UNFOCUS,
    EVENT_WINDOW_MINIMIZE,
    EVENT_WINDOW_RESTORE,
    EVENT_WINDOW_DPI_CHANGE,
    EVENT_TOUCH,   // multitouch via WM_TOUCH
    EVENT_POINTER, // stylet / pointer (WM_POINTER*)
    EVENT_RAW
} EventType;

/// @brief structure pour stocker les events sous un format commun
typedef struct
{
    EventType type;
    union
    {
        struct
        { // ---------------- Clavier
            unsigned int keycode;
            unsigned char shift, ctrl, alt, win;
            unsigned char extended;
            unsigned short repeat;
            unsigned char pressed_before;
        } key;

        struct
        {               // ---------------- Souris
            int x, y;   // position absolue
            int dx, dy; // delta relatif
            int button; // 0=left, 1=right, 2=middle, 3/4=x1/x2
            int delta;  // molette
        } mouse;

        struct
        { // ---------------- Fenêtre
            int width, height;
        } window;

        struct
        { // ---------------- Multitouch basique
            int count;
            struct
            {
                int id, x, y;
            } points[MAX_TOUCH_POINTS];
        } touch;

        struct
        { // ---------------- Stylet / Pointer avancé
            int x, y;
            int pressure; // 0-1024 typiquement
            int tilt_x;   // inclinaison X en degrés
            int tilt_y;   // inclinaison Y en degrés
            int rotation; // rotation de la pointe (si dispo)
            int pointer_id;
            int pointer_type; // PT_TOUCH, PT_PEN, PT_MOUSE
        } pointer;
    };

    // Info brute pour debug
    char rawEvent[MAX_RAW_EVENT_SIZE];
    unsigned long rawSize;

} Event;

// queue d'event
typedef struct
{
    Event queue[MAX_EVENTS];
    int start;
    int end;
} EventQueue;

static int last_mouse_x = 0;
static int last_mouse_y = 0;

extern EventQueue eventQueue;

void platformInitWindow(char *windowName, int width, int height);
void platformPollEventWindow(void);
void platformCloseWindow(void);

unsigned char platformIsRunning(void);

#endif
