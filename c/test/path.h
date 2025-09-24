#ifndef path_H
#define path_H

/**
 * @brief obtient le dossier de l'executable
 *
 * @param path le chemin ou on est
 *
 * @return la taille du chemin
 */
unsigned short getCurrentPath(char *path);

/**
 * @brief renvoie si le chemin existe
 *
 * @param path le chemin
 *
 * @return si le chemin existe
 */
unsigned char pathExists(const char *path);

/**
 * @brief cree le chemin passer si il n'existe pas
 *
 * @param path le chemin
 * @param lastIsFolder si le dernier element est un dossier
 */
void createPathIfNotExists(char *path, unsigned char lastIsFolder);

/**
 * @brief normalize le chemin au format /
 *
 * @param path le chemin a normalizer
 */
void normalizePathToSlash(char *path);

#endif