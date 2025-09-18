#ifndef MotorManager_H
#define MotorManager_H

#include "Motor.h"
#include "Direction.h"

namespace motor
{
    class MotorManager
    {
    private:
        Motor motorRight = {5, 7};
        Motor motorLeft = {6, 8};
        const int PINControlMotor = 3; // si moteur roule ou pas
        bool control = false;

    public:
        const int maxSpeed = 255; // la vitesse max
        const int minSpeed = 50;  // la vitesse min

        MotorManager();
        ~MotorManager();
        /**
         * @brief reset tout les ports sur leur valeur par default et desactive le control
         *
         * - Speed = 0
         *
         * - Dir = HIGH
         */
        void ResetMove();
        /**
         * @brief Contrôle le mouvement des moteurs pendant une durée donnée ou a l'infini.
         *
         * @param directionRight  Direction du moteur A (true = avant, false = arrière)
         * @param speedRight      Vitesse du moteur A (0; 50 à 255)
         * @param directionLeft  Direction du moteur B (true = avant, false = arrière)
         * @param speedLeft      Vitesse du moteur B (0; 50 à 255)
         * @param time        Durée du mouvement en millisecondes default: 0 -> infini
         */
        void Move(bool directionRight, unsigned char speedRight, bool directionLeft, unsigned char speedLeft, unsigned int time = 0);
        /**
         * @brief deplacement en fonction de la direction
         *
         * @param speed Vitesse du moteur droite
         * @param relation Raport entre les vitesses des moteurs default: 2
         * @param time Durée du mouvement en millisecondes default: 0 -> infini
         */
        void MoveByDirection(Direction direction, unsigned char speed, unsigned char relation = 2, unsigned int time = 0);
        /**
         * @brief change la vitesse du moteur indiquer
         *
         * @param speed la nouvelle vitesse
         * @param motor le moteur conserner
         *
         * -0x01 droit
         *
         * -0x02 gauche
         *
         * -0x03 les deux
         */
        void ChangeSpeed(unsigned char speed, unsigned char motor);
        /**
         * @brief change la direction du moteur indiquer
         *
         * @param direction la nouvelle direction
         * @param motor le moteur conserner
         *
         * -0x01 droit
         *
         * -0x02 gauche
         *
         * -0x03 les deux
         */
        void ChangeDirection(bool direction, unsigned char motor);
        /**
         * @brief change la direction du tank
         *
         * @param direction la nouvelle direction
         */
        void ChangeTankDirection(Direction direction);

        /**
         * @brief gere les moteurs
         *
         * @param size la taille de l'input
         * @param inpur les consignes
         *
         * @return une valeur d'erreur
         *
         * - 0x00 pas d'erreur
         *
         * - 0x02 pas trouve
         *
         * - 0x03 taille input invalide
         *
         * - 0x04 valeur incorect
         */
        unsigned char ManageMotor(unsigned short size, char *input);
    };

} // namespace motor

#endif