#include "PrefixManager.h"

namespace prefix
{
    PrefixManager::PrefixManager() {}
    PrefixManager::PrefixManager(PrefixEntry *entries, unsigned short count)
    {
        for (unsigned short i = 0; i < count; i++)
        {
            addPrefix(entries[i]);
        }
    }
    PrefixManager::~PrefixManager() {}
    bool PrefixManager::addPrefix(PrefixEntry entrie)
    {
        return prefixMap.put(entrie.key, entrie.value);
    }
    bool PrefixManager::getPrefix(const char *prefix, managerType &manager)
    {
        return prefixMap.get(prefix, manager);
    }
    bool PrefixManager::hasPrefix(char *prefix)
    {
        return prefixMap.exist(prefix);
    }
    bool PrefixManager::removePrefix(const char *prefix, bool secure)
    {
        return prefixMap.remove(prefix, secure);
    }
} // namespace prefix
