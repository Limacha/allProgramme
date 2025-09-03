template <typename Key, typename Value, typename KeyCompare>
bool DynamicMap<Key, Value, KeyCompare>::put(const Key &key, const Value &value)
{
    // Cherche si la clé existe déjà
    for (int i = 0; i < capacity; i++)
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
    for (int i = 0; i < capacity; i++)
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
    int newCapacity = (capacity == 0) ? 2 : capacity * 2;
    Entry *newEntries = new Entry[newCapacity];
    for (int i = 0; i < capacity; i++)
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
bool DynamicMap<Key, Value, KeyCompare>::remove(const Key &key, bool secure)
{
    for (int i = 0; i < capacity; i++)
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
Value DynamicMap<Key, Value, KeyCompare>::get(const Key &key, const Value &defaultValue)
{
    for (int i = 0; i < capacity; i++)
    {
        if (entries[i].used)
        {
            if (comp(entries[i].key, key))
            {
                return entries[i].value;
            }
        }
    }
    return defaultValue;
}