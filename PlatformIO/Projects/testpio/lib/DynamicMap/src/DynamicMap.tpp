template <typename Key, typename Value, typename KeyCompare>
DynamicMap<Key, Value, KeyCompare>::DynamicMap()
{
}

template <typename Key, typename Value, typename KeyCompare>
DynamicMap<Key, Value, KeyCompare>::DynamicMap(KeyVal *defaultTab, unsigned int size)
{
    for (unsigned int i = 0; i < size; i++)
    {
        put(defaultTab[i].key, defaultTab[i].value);
    }
}

template <typename Key, typename Value, typename KeyCompare>
DynamicMap<Key, Value, KeyCompare>::~DynamicMap()
{
    delete[] entries;
}

template <typename Key, typename Value, typename KeyCompare>
bool DynamicMap<Key, Value, KeyCompare>::put(const Key key, const Value &value)
{
    // Cherche si la clé existe déjà
    for (unsigned int i = 0; i < capacity; i++)
    {
        if (entries[i].used)
        {
            if (comp(entries[i].key, key))
            {
                entries[i].value = value;
                return true;
            }
        }
    }

    // Chercher un emplacement libre
    for (unsigned int i = 0; i < capacity; i++)
    {
        if (!entries[i].used)
        {
            entries[i].key = key;
            entries[i].value = value;
            entries[i].used = true;
            count++;
            return true;
        }
    }

    // Agrandir si aucun emplacement libre
    unsigned int newCapacity = (capacity == 0) ? 2 : capacity * 2;
    Entry *newEntries = new Entry[newCapacity];
    for (unsigned int i = 0; i < capacity; i++)
    {
        newEntries[i] = entries[i];
    }
    delete[] entries;
    entries = newEntries;
    capacity = newCapacity;

    // Ajouter dans le premier nouvel emplacement
    entries[count].key = key;
    entries[count].value = value;
    entries[count].used = true;
    count++;
    return true;
}

template <typename Key, typename Value, typename KeyCompare>
bool DynamicMap<Key, Value, KeyCompare>::remove(const Key key, bool secure)
{
    for (unsigned int i = 0; i < capacity; i++)
    {
        if (entries[i].used && comp(entries[i].key, key))
        {
            if (secure)
            {
                entries[i].key = Key();
                entries[i].value = Value();
            }
            entries[i].used = false;
            count--;
            return true;
        }
    }
    return false;
}

template <typename Key, typename Value, typename KeyCompare>
bool DynamicMap<Key, Value, KeyCompare>::get(const Key key, Value &outValue)
{
    for (unsigned int i = 0; i < capacity; i++)
    {
        if (entries[i].used)
        {
            if (comp(entries[i].key, key))
            {
                outValue = entries[i].value;
                return true;
            }
        }
    }
    return false;
}

template <typename Key, typename Value, typename KeyCompare>
bool DynamicMap<Key, Value, KeyCompare>::exist(Key key)
{
    for (unsigned int i = 0; i < capacity; i++)
    {
        if (entries[i].used)
        {
            if (comp(entries[i].key, key))
            {
                return true;
            }
        }
    }
    return false;
}