
void u16To2Digits(char *dest, unsigned short value)
{
    dest[0] = (value / 10) + '0';
    dest[1] = (value % 10) + '0';
}

void u16To4Digits(char *dest, unsigned short value)
{
    dest[0] = (value / 1000) % 10 + '0';
    dest[1] = (value / 100) % 10 + '0';
    dest[2] = (value / 10) % 10 + '0';
    dest[3] = (value % 10) + '0';
}

unsigned int strLen(const char *s)
{
    unsigned int n = 0;
    // si s existe et valeurs != \0
    while (s && s[n])
        n++;
    return n;
}
