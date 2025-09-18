#ifndef SerialController_H
#define SerialController_H

#include <Arduino.h>

#include <Function.h>

namespace serial
{
    class SerialController
    {
    public:
        /**
         * @brief ouvre le port serie si il n'est pas deja ouvert
         *
         * @param bauds les bauds sur le quel l'ouvrir
         *
         * @return si il on a put l'ouvrir ou pas
         */
        bool open(unsigned long bauds);

        /**
         * @brief ferme le port serie si ouvert
         *
         * @return si on a put le fermer
         */
        bool close();

        /**
         * @brief permet d'envoyer des chars sur le port serie
         *
         * @param chars les chars a envoyer
         * @param size le nombre de chart a afficher
         *
         * @return si on a put envoye les chars
         */
        bool write(const char *chars, unsigned short size);

        /**
         * @brief permet d'envoyer des chars sur le port serie
         *
         * @param chars les chars a envoyer
         * @param size le nombre de chart a afficher
         *
         * @return si on a put envoye les chars
         */
        template <typename T>
        bool print(T val);

        /**
         * @brief lit un nombre de char
         *
         * @param nb nombre de char a lire
         * @param timeMax temp maximum entre chaque reception de char default: 0
         * @param delayMax temp maximum pour la reception total default: 0
         *
         * @return renvoye les chars lu
         */
        bool read(char **outBuffer, unsigned short &nb, unsigned short timeMax = 0, unsigned short delayMax = 0);

        /**
         * @brief li tout les characteres dispo
         *
         * @param outSize renvoie le nombre de char lu
         * @param delay delay entre chaque lecture
         *
         * @return renvoie tout les characteres lu
         */
        char *readAll(unsigned short &outSize, unsigned char delay = 2);

        /**
         * @brief delete tous les donnes dispo
         *
         * @param delay delay entre chaque supression
         *
         * @return true si plus rien n'est dispo
         */
        bool deleteAll(unsigned char latency = 2);

        /**
         * @brief verifie si il y a qq chose a lire
         *
         * @return true si quelque chose est dispo a la lecture
         */
        bool dispo();

        /**
         * @brief regarde le nombre d'element dispo a la lecture
         *
         * @return retourne le nombre d'element disponible
         */
        int available();

        /**
         * @brief envoie le delay en ms depuis start
         *
         * @param le debutdu chrono
         */
        void showDelay(unsigned long start, bool endLigne = false);
    };
#include "SerialController.tpp"
} // namespace serial;
#endif