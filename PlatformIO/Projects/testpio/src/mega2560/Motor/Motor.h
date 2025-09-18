#ifndef Motor_H
#define Motor_H

#include "MotorDriver.h"

namespace motor
{
    class Motor
    {
    private:
        // le pin sur le quel la vitesse est envoyer
        unsigned char pinSpeed;
        // le pin sur le quel la direction est envoyer
        unsigned char pinDirection;
        // la vitesse du moteur
        unsigned char speed = 0;
        // la direction du moteur
        bool direction = false;

    public:
        const unsigned char Speed() { return speed; };
        const bool Direction() { return direction; };
        /**
         * @brief cree un nouveau moteur
         *
         * @param pinSpeed le pin de la vitesse
         * @param pinDirection le pin de la direction
         */
        Motor(unsigned char pinSpeed, unsigned char pinDirection);
        ~Motor();
        /**
         * @brief init tout les ports en output
         */
        void init();
        /**
         * @brief defini la vitesse de rotation
         *
         * @param speed la vitesse de rotation
         */
        void setSpeed(unsigned char speed);
        /**
         * @brief defini la direction du moteur
         *
         * @param directon la direction choisis
         */
        void setDirection(bool direction);
        /**
         * @brief reset tout a 0
         */
        void reset();
    };

} // namespace motor

#endif