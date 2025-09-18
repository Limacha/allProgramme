#ifndef log_H
#define log_H

/**
 * @brief set le chemin du fichier principal et fais un test
 */
void initLog(void);

/**
 * @brief ajoute un contenu au fichier buffer
 *
 * @param data les donner a ajouter
 * @param dateTime ajoute la date et l'heure ou pas
 *
 * @return si l'ajout au fichier a put se faire
 */
unsigned char addToLog(char *buffer, unsigned char dateTime);

/**
 * @brief ajout la date au fichier log
 *
 * @param endLine si on retourne a la ligne apres
 *
 * @return si l'ajout a put se faire
 */
static unsigned char addDateToLog(unsigned char endLine);

/**
 * @brief ajout l'heure au fichier log
 *
 * @param endLine si on retourne a la ligne apres
 *
 * @return si l'ajout a put se faire
 */
static unsigned char addTimeToLog(unsigned char endLine);

/**
 * @brief obtient le chemin du dossier log principal
 *
 * @param outPath le chemin
 *
 * @return la taille du chemin
 */
unsigned short getLogPath(char *outPath);

/**
 * @brief ajoute un contenu au log
 *
 * @param data les donner a ajouter
 *
 * @return si l'ajout s'est bien passer
 */
unsigned char addContentToLog(char *data);

/**
 * @brief ajoute un contenu au log
 *
 * @param data les donner a ajouter
 * @param size la quantiter a ajouter
 *
 * @return si l'ajout s'est bien passer
 */
unsigned char addSizedContentToLog(char *data, unsigned long size);

#endif