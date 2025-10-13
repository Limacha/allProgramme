#ifndef string_H
#define string_H

/// @brief une struct qui permet de faire des chaine de char dynamique
typedef struct
{
    char *data;
    unsigned int len; // nombre de char sans \0
    unsigned int cap; // capacité allouée
} String;

/**
 * @brief creez un string sur base d'une chaine
 *
 * @param initial une chaine de base pour le string
 *
 * @return un nouveau string (a liberer avec freeString)
 */
String stringCreate(const char *initial);

/**
 * @brief libere l'espace d'un string et set tout a 0
 *
 * @param s le string a detruire
 */
void stringFree(String *s);

/**
 * @brief ajoute du text a un string
 *
 * @param s le string au quel ajouter le text
 * @param text le text a ajouter
 */
void stringAppend(String *s, const char *text);

/**
 * @brief addition deux string ensemble
 *
 * @param a le premier string
 * @param b le deuxieme string a ajouter au premier
 *
 * @return le nouveau string cree (a liberer)
 */
String stringAdd(const String *a, const String *b);

/**
 * @brief cree un string avec une partie d'un autre string
 *
 * @param a le string de base
 * @param start ou on commence a copier
 * @param end la ou on s'arrete (en partant de la fin)
 *
 * @return un nouveau string (a liberer)
 */
String stringSlice(const String *a, const unsigned int start, unsigned int end);

#endif