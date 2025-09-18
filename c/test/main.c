#include "logic.h"
#include "input.h"
#include "render.h"
#include "files.h"
#include "log/log.h"

int main(void)
{
    initLog();
    initLogic(800, 600);  // créer une surface 800x600 pixels
    initRender(800, 600); // init rendu selon OS
    initInput();          // init input selon OS

    DirList racineContent = getDirContent("C:/Users/Nico/Documents/github/allProgramme/c/test");
    char *buffer = dirListToSingleBuffer(&racineContent, "\n-> ", 0x01);
    if (buffer)
    {
        addToLog(buffer, 0x02); // utiliser le buffer
        memoryFree(buffer);
        // libérer le buffer après utilisation
    }
    addToLog("\n", 0x00);

    while (isRunning())
    {
        processInput();           // récupérer les entrées
        updateLogic();            // mettre à jour pixels
        renderFrame(getPixels()); // afficher pixels
    }

    shutdownRender();
    shutdownLogic();
    return 0;
}
