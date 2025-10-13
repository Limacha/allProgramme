#ifndef fonction_H
#define fonction_H

void u16To2Digits(char *dest, unsigned short value);
void u16To4Digits(char *dest, unsigned short value);

/**
 * @brief permet de calculer la taille d'une chaine
 *
 * @param s la chaine a calculer
 *
 * @return le nombre de charactere
 */
unsigned int strLen(const char *s);

#endif