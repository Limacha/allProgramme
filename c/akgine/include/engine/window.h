#ifndef WINDOW_H
#define WINDOW_H

/**
 * initialise la fenetre
 *
 * @param windowName - nom de la fenetre
 * @param width - largeur de la fenetre
 * @param height - hauteur de la fenetre
 */
void initWindow(char *windowName, int width, int height);

/**
 * recupere tout les events du system envoyer par/pour la fenetre et les renvoie la ou elle doive etre traites
 */
void pollEventWindow(void);

void closeWindow(void);

/**
 * obtient pour savoir si la fenetre tourne encore ou pas
 *
 * @return si la fenetre tourne
 */
unsigned char isRunning(void);

#endif
