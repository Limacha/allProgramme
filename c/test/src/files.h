#ifndef files_H
#define files_H

typedef struct
{
    char **items; // tableau de chemins
    unsigned int count;
} DirList;

/**
 * @brief cree/reecrit un fichier avec le contenu fournit
 *
 * @param path le chemin du fichier
 * @param data le contenu du fichier
 * @param size la taille du contenu
 *
 * @return si le fichier a put etre ecrit
 */
unsigned char writeFile(const char *path, const char *data, unsigned long size);

/**
 * @brief ajoute du contenu a un fichier
 *
 * @param path le chemin du fichier
 * @param data le contenu a ajouter
 *
 * @return si tout s'est bien passer
 */
unsigned char addToFile(const char *path, const char *data);

/**
 * @brief lin un fichier et renvouie un buffer dynamique
 *
 * @param path le chemin du fichier
 * @param outSize taille du buffer sorti
 *
 * @return toute les donnes du fichier (a liberer)
 */
void *readFileBinary(const char *path, unsigned long *outSize);

/**
 * @brief libere le buffer contenant le contenu d'un fichier
 *
 * @param buffer le contenu a liberer
 */
void freeFileBinary(void *buffer);

/**
 * @brief renvoie si le chemin existe et que s'est un fichier
 *
 * @param path le chemin
 *
 * @return si le fichier existe
 */
unsigned char fileExists(const char *path);

/**
 * @brief renvoie si le chemin existe et que s'est un dossier
 *
 * @param path le chemin
 *
 * @return si le dossier existe
 */
unsigned char dirExists(const char *path);

/**
 * @brief cree un dossier
 *
 * @param path le chemin
 *
 * @return si il a ete cree
 */
unsigned char createDir(const char *path);

/**
 * @brief liste le contenu du dossier
 *
 * @param path le chemin a lister
 *
 * @return la liste
 */
DirList getDirContent(const char *path);

/**
 * @brief transforme DirList en un seul tableau
 *
 * @param list la list a transformer
 * @param separator contenu entre chaque chemin
 * @param startEnd ajout du separateur au debut ou a la fin bit0 = start, bit1 = end
 *
 * @return un tableau avec tout les chemins (a liberer)
 */
char *dirListToSingleBuffer(DirList *list, char *separator, unsigned char startEnd);
#endif