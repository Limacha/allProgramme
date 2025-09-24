#include "log.h"
#include "../files.h"
#include "../dateTime.h"

unsigned char addToLog(char *path, char *data, unsigned char dateTime)
{
    createPathIfNotExists(path, 0x00);
    if (dateTime)
    {
        char *date = createDateChain(0x00);
        char *time = createTimeChain(0x00);
        addToFile(path, date);
        addToFile(path, time);
        addToFile(path, " ");
    }

    return addToFile(path, data);
}
