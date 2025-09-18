#include "log.h"
#include "../string.h"
#include "../files.h"
#include "../dateTime.h"
#include "../fonction.h"

String path;

void initLog(void)
{
    char currentPath[MAX_PATH_LENGHT];
    getCurrentPath(currentPath);
    stringAppend(&path, currentPath);
    stringAppend(&path, "/log/log.aklog");
    normalizePathToSlash(path.data);

    addToLog(path.data, 0x03);
    addToLog("\n", 0x00);
}

unsigned char addContentToLog(char *data)
{
    return addToLog(data, 0xFF);
}

unsigned char addSizedContentToLog(char *data, unsigned long size)
{
    String sData = {data};
    stringSlice(&sData, 0, size);
    return addToLog(data, 0xFF);
}

static unsigned char addToLog(char *data, unsigned char dateTime)
{
    createPathIfNotExists(path.data, 0x00);
    if (dateTime)
    {
        addDateToLog(0x00);
        addTimeToLog(0x00);
        addToFile(path.data, " ");
    }

    return addToFile(path.data, data);
}

static unsigned char addDateToLog(unsigned char endLine)
{
    unsigned short year, month, day;
    getDate(&year, &month, &day);
    char date[13];
    u16To4Digits(date, year);
    date[4] = '-';
    u16To2Digits(date + 5, month);
    date[7] = '-';
    u16To2Digits(date + 8, day);
    date[10] = ' ';
    if (endLine)
    {
        date[11] = '\n';
        date[12] = '\0';
    }
    date[11] = '\0';
    return addToLog(date, 0x00);
}

static unsigned char addTimeToLog(unsigned char endLine)
{
    unsigned short hour, minute, second;
    getTime(&hour, &minute, &second);
    char time[11];
    u16To2Digits(time, hour);
    time[2] = ':';
    u16To2Digits(time + 3, minute);
    time[5] = ':';
    u16To2Digits(time + 6, second);
    time[8] = ' ';
    if (endLine)
    {
        time[9] = '\n';
        time[10] = '\0';
    }
    time[9] = '\0';
    return addToLog(time, 0x00);
}

unsigned short getLogPath(char *outPath)
{
    outPath = path.data;
    return path.len;
}
