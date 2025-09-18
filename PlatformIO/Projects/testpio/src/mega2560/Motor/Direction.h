#ifndef Direction_H
#define Direction_H

namespace motor
{
    enum class Direction // direction emprunter
    {
        Forward,       // avancer
        Backward,      // reculer
        Left,          // tourner vers la gauche
        Right,         // tourner vers la droite
        LeftForward,   // avancr en allant a gauche
        LeftBackward,  // reculer en allant a gauche
        RightForward,  // avancer en allant a droite
        RightBackward, // reculer en allant a droite
        stop_it        // s'arreter
    };
} // namespace motor

#endif