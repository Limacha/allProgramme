#include "path.h"
#include "platform/platform.h"
#include "files.h"

unsigned short getCurrentPath(char *path)
{
    return platformGetCurrentPath(path);
}

unsigned char pathExists(const char *path)
{
    return platformPathExists(path);
}

void createPathIfNotExists(char *path, unsigned char lastIsFolder)
{
    normalizePathToSlash(path);
    unsigned short n = 0;
    while (path[n] != '\0')
    {
        if (path[n] == '/')
        {
            char saved = path[n];
            path[n] = '\0';
            if (!dirExists(path))
            {
                createDir(path);
            }
            path[n] = saved;
        }
        n++;
    }
    if (lastIsFolder)
    {
        if (!dirExists(path))
            createDir(path);
    }
    else
    {
        if (!fileExists(path))
            writeFile(path, "", 0);
    }
}

/**
 * @brief normalize le chemin avec des /
 *
 * @param path chemin avec \0 a la fin
 */
void normalizePathToSlash(char *path)
{
    if (!path)
        return;

    unsigned int i = 0;
    while (path[i] != '\0')
    {
        if (path[i] == '\\')
            path[i] = '/';
        i++;
    }
}