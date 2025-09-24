
String path;

void initLog(void)
{
    char *currentPath;
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
    unsigned char result = addToLog(sData.data, 0xFF);

    stringFree(&sData);
    return result;
}
unsigned short getLogPath(char *outPath)
{
    outPath = path.data;
    return path.len;
}
