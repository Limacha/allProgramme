#ifndef PrefixManager_H
#define PrefixManager_H

#include <DynamicMap.h>

namespace prefix
{
    typedef unsigned char (*managerType)(unsigned short size, char *input);
    struct PrefixEntry
    {
        const char *key;
        managerType value;
    };
    class PrefixManager
    {
    private:
        DynamicMap<const char *, managerType> prefixMap;

    public:
        const unsigned short prefixSize = 3;
        PrefixManager();
        PrefixManager(PrefixEntry *entries, unsigned short count);
        ~PrefixManager();
        bool addPrefix(PrefixEntry prefix);
        bool getPrefix(const char *prefix, managerType &manager);
        bool hasPrefix(char *prefix);
        bool removePrefix(const char *prefix, bool secure = false);
    };

} // namespace prefix
#endif