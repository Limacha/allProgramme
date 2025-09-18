#include "dateTime.h"
#include "platform/platform.h"

void getDate(unsigned short *year, unsigned short *month, unsigned short *day)
{
    platformGetDate(year, month, day);
}
void getTime(unsigned short *hour, unsigned short *minute, unsigned short *second)
{
    platformGetTime(hour, minute, second);
}