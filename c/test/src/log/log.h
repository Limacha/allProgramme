#ifndef log_H
#define log_H

/**
 * @brief ajoute un contenu au fichier buffer
 *
 * @param data les donner a ajouter
 * @param dateTime ajoute la date et l'heure ou pas
 *
 * @return si l'ajout au fichier a put se faire
 */
unsigned char addToLog(char *path, char *data, unsigned char dateTime);

#endif