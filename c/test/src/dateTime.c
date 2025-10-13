#include "dateTime.h"
#include "platform/platform.h"
#include "fonction.h"

void getDate(unsigned short *year, unsigned short *month, unsigned short *day)
{
    platformGetDate(year, month, day);
}

void getTime(unsigned short *hour, unsigned short *minute, unsigned short *second)
{
    platformGetTime(hour, minute, second);
}

char *createDateChain()
{
    unsigned short year, month, day;
    getDate(&year, &month, &day);
    char date[12];
    u16To4Digits(date, year);
    date[4] = '-';
    u16To2Digits(date + 5, month);
    date[7] = '-';
    u16To2Digits(date + 8, day);
    date[10] = ' ';
    /*if (endLine)
    {
        date[11] = '\n';
        date[12] = '\0';
    }*/
    date[10] = '\0';
    return date;
}

char *createTimeChain()
{
    unsigned short hour, minute, second;
    getTime(&hour, &minute, &second);
    char time[10];
    u16To2Digits(time, hour);
    time[2] = ':';
    u16To2Digits(time + 3, minute);
    time[5] = ':';
    u16To2Digits(time + 6, second);
    time[8] = ' ';
    /*if (endLine)
    {
        time[9] = '\n';
        time[10] = '\0';
    }*/
    time[9] = '\0';
    return time;
}
