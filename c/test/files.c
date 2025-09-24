#include "files.h"
#include "platform/platform.h"
#include "memory.h"
#include "fonction.h"

unsigned char writeFile(const char *path, const char *data, unsigned long size)
{
    return platformWriteFile(path, data, size);
}

unsigned char addToFile(const char *path, const char *data)
{
    return platformAppendToFile(path, data);
}

void *readFileBinary(const char *path, unsigned long *outSize)
{
    return platformReadFileBinary(path, outSize);
}

void freeFileBinary(void *buffer)
{
    platformFreeFileBinary(buffer);
}

unsigned char fileExists(const char *path)
{
    return platformFileExists(path);
}

unsigned char dirExists(const char *path)
{
    return platformDirExists(path);
}

unsigned char createDir(const char *path)
{
    return platformCreateDir(path);
}

DirList getDirContent(const char *path)
{
    DirList result = {0};
    unsigned int count = 0;
    result.items = platformListDir(path, &count);
    result.count = count;
    return result;
}

char *dirListToSingleBuffer(DirList *list, char *separator, unsigned char startEnd)
{
    if (!list || list->count == 0)
        return 0;

    unsigned short separatorLenght = strLen(separator);

    // Calculer la taille totale
    unsigned int totalLen = 0;
    for (unsigned int i = 0; i < list->count; i++)
    {
        char *path = list->items[i];
        unsigned short pathLenght = strLen(path);
        totalLen += pathLenght;
    }
    if (startEnd & 0x01)
        totalLen += separatorLenght;
    if (startEnd & 0x02)
        totalLen += separatorLenght;

    totalLen += (list->count - 1) * separatorLenght; // ajoute la taille du separateur a chaque fois
    totalLen += 1;                                   // +1 pour le '\0' final

    char *buffer = (char *)memoryMalloc(totalLen);
    if (!buffer)
        return 0;

    unsigned int k = 0;
    // ajoute le separateur au debut
    if (startEnd & 0x01)
    {
        for (unsigned short i = 0; i < separatorLenght; i++)
        {
            buffer[i] = separator[i];
        }
        k = separatorLenght;
    }
    // Copier les chaînes dans le buffer
    for (unsigned int i = 0; i < list->count; i++)
    {
        char *s = list->items[i];
        unsigned int j = 0;
        while (s[j])
        {
            buffer[k++] = s[j++];
        }

        if (i != list->count - 1) // ajouter le separator sauf après le dernier
            for (unsigned short i = 0; i < separatorLenght; i++)
                buffer[k++] = separator[i];
    }
    // ajoute le separateur a la fin
    if (startEnd & 0x02)
        for (unsigned short i = 0; i < separatorLenght; i++, k++)
            buffer[k] = separator[i];

    buffer[k] = '\0'; // terminer la chaîne
    return buffer;
}
