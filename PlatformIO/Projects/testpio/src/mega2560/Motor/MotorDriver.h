#ifndef MotorDriver_H
#define MotorDriver_H

#include <Arduino.h>

namespace motor
{
    struct MotorDriver
    {
        /**
         * @brief init tout les ports en output
         *
         * @param pinSpeed le pin sur le quel la vitesse sera envoye
         * @param pinDirection le pin sur le quel la direction sera envoye
         */
        static void init(unsigned char pinSpeed, unsigned char pinDirection);
        /**
         * @brief defini la vitesse de rotation
         *
         * @param pinSpeed le pin sur le quel la vitesse est envoyer
         * @param speed la vitesse de rotation
         */
        static void setSpeed(unsigned char pinSpeed, unsigned char speed);
        /**
         * @brief defini la direction du moteur
         *
         * @param pinDirection le pin sur le quel la direction est envoyer
         * @param directon la direction choisis
         */
        static void setDirection(unsigned char pinDirection, bool direction);
        /**
         * @brief reset tout a 0
         *
         * @param pinSpeed le pin sur le quel reset la vitesse
         * @param pinDirection le pin sur le quel reset la direction
         */
        static void reset(unsigned char pinSpeed, unsigned char pinDirection);
    };

} // namespace motor

#endif