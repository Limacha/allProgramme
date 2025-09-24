
/**
 * @brief set le chemin du fichier principal et fais un test
 */
void initLog(void);

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

/**
 * @brief obtient le chemin du dossier log principal
 *
 * @param outPath le chemin
 *
 * @return la taille du chemin
 */
unsigned short getLogPath(char *outPath);
